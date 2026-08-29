// Package dssinterface This file contains the
package dssinterface

import (
	"context"
	"crypto/tls"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"regexp"
	"strconv"
	"syscall"
	"time"

	"github.com/antihax/optional"
	"github.com/at-wat/mqtt-go"
	"github.com/davecgh/go-spew/spew"
	"github.com/xgcssch/DigitalstromMQTTBridge/internal/pkg/swagger"

	"k8s.io/klog/v2"
)

const messageReceiveTimeout = 120 * time.Second
const debugmode = false

// Backoff bounds used when a network operation (dSS or MQTT) fails. Instead of
// terminating the process on a transient connectivity problem the bridge keeps
// retrying, waiting a bit longer after every consecutive failure.
const (
	retryInitialWait = 2 * time.Second
	retryMaxWait     = 60 * time.Second
)

// consecutiveEventFailures is the number of failed event polls that are
// tolerated before the current dSS session is dropped and re-established.
const consecutiveEventFailures = 3

// retryWait blocks for the given backoff duration, returning early if ctx is
// cancelled, and returns the next (doubled, capped) backoff value.
func retryWait(ctx context.Context, current time.Duration) time.Duration {
	t := time.NewTimer(current)
	defer t.Stop()
	select {
	case <-ctx.Done():
	case <-t.C:
	}
	next := current * 2
	if next > retryMaxWait {
		next = retryMaxWait
	}
	return next
}

type HomeassistantDevicesAdvertisment struct {
	Ids []string `json:"ids"`
}
type HomeassistantSwitchAdvertisment struct {
	Name       string                           `json:"name"`
	StatT      string                           `json:"stat_t"`
	AvtyT      string                           `json:"avty_t"`
	PlAvail    string                           `json:"pl_avail"`
	PlNotAvail string                           `json:"pl_not_avail"`
	CmdT       string                           `json:"cmd_t"`
	ValTpl     string                           `json:"val_tpl"`
	PlOff      string                           `json:"pl_off"`
	PlOn       string                           `json:"pl_on"`
	StatOff    string                           `json:"stat_off"`
	StatOn     string                           `json:"stat_on"`
	UniqId     string                           `json:"uniq_id"`
	Dev        HomeassistantDevicesAdvertisment `json:"dev"`
}

func publishHomeassistantAdvertisments(
	ctx context.Context,
	ac swagger.APIClient,
	MQTTClient mqtt.Client) {

	for {
		if zones, ok := readApartmentZones(ctx, ac); ok {
			publishZoneAdvertisments(ctx, MQTTClient, zones)
		}

		select {
		case <-ctx.Done():
			return
		case <-time.After(time.Minute):
		}
	}
}

// readApartmentZones fetches the apartment structure from the dSS, returning
// ok == false (instead of panicking or aborting the process) whenever the dSS
// is unreachable or answers with an incomplete document.
func readApartmentZones(
	ctx context.Context,
	ac swagger.APIClient) ([]swagger.InlineResponse2002ResultApartmentZones, bool) {

	structure, _, err := ac.ApartmentApi.GetStructure(ctx)
	if err != nil {
		klog.Warningf("publishHomeassistantAdvertisments: unable to read structure from dSS: %v", err)
		return nil, false
	}
	if structure.Result == nil || structure.Result.Apartment == nil {
		klog.Warningf("publishHomeassistantAdvertisments: dSS returned an incomplete structure")
		return nil, false
	}
	return structure.Result.Apartment.Zones, true
}

func publishZoneAdvertisments(
	ctx context.Context,
	MQTTClient mqtt.Client,
	zones []swagger.InlineResponse2002ResultApartmentZones) {

	for _, v := range zones {
		if v.Id == 0 {
			continue
		}

		for _, g := range v.Groups {
			if (g.ApplicationType != 1) ||
				v.Name == "" {
				continue
			}

			uu := HomeassistantSwitchAdvertisment{
				Name:       v.Name,
				StatT:      fmt.Sprintf("stat/dssBridge/group/%d/%d", v.Id, g.ApplicationType),
				AvtyT:      "tele/dssBridge/LWT",
				PlAvail:    "Online",
				PlNotAvail: "Offline",
				CmdT:       fmt.Sprintf("cmnd/dssBridge/group/%d/%d", v.Id, g.ApplicationType),
				ValTpl:     "{{value_json.scene}}",
				PlOff:      "0",
				PlOn:       "5",
				StatOff:    "0",
				StatOn:     "5",
				UniqId:     fmt.Sprintf("dssBridge_%d_%d", v.Id, g.ApplicationType),
				Dev: HomeassistantDevicesAdvertisment{
					Ids: []string{fmt.Sprintf("dssBridge_%d_%d", v.Id, g.ApplicationType)}},
			}

			pl, _ := json.Marshal(uu)
			if err := MQTTClient.Publish(ctx, &mqtt.Message{
				Topic:   fmt.Sprintf("homeassistant/switch/dssBridge/%d_%d/config", v.Id, g.ApplicationType),
				QoS:     mqtt.QoS1,
				Payload: pl,
				Retain:  false,
			}); err != nil {
				klog.Warningf("publishHomeassistantAdvertisments: MQTT publish failed: %v", err)
				return
			}

			klog.V(3).Infof("Published Homeassistant discovery info for Zone %d: %s", v.Id, v.Name)
		}
	}
}

// Configurationdata
type DssBridgeConfiguration struct {
	BaseURL  string
	Username string
	Password string
	MQTTHost string
	MQTTPort int
}

// StartDssBridge Start listening on the UDP Port 22460 for Monitoring packets from the heat control
func StartDssBridge(
	configuration DssBridgeConfiguration) {

	tlsconfig := tls.Config{InsecureSkipVerify: true}

	tr := &http.Transport{
		MaxIdleConns:          10,
		IdleConnTimeout:       messageReceiveTimeout,
		ResponseHeaderTimeout: messageReceiveTimeout,
		TLSClientConfig:       &tlsconfig,
	}
	client := &http.Client{Transport: tr}

	sc := swagger.NewConfiguration()
	sc.HTTPClient = client

	sigs := make(chan os.Signal, 1)
	done := make(chan bool, 1)

	signal.Notify(sigs, syscall.SIGINT, syscall.SIGTERM)

	BaseContext, cancel := context.WithCancel(context.Background())

	go func(cancel context.CancelFunc) {
		<-sigs

		cancel()

		done <- true
	}(cancel)

	go func() {
		// Inner completion channel
		InnerDone := make(chan bool, 1)

		// Prepare connection to dSS
		var ac = swagger.NewAPIClient(sc)
		ac.ChangeBasePath(configuration.BaseURL + "/json")

		// Prepare the Connection to the MQTT Server
		MQTTClient, err := mqtt.NewReconnectClient(
			// Dialer to connect/reconnect to the server.
			&mqtt.URLDialer{
				URL: fmt.Sprintf("mqtt://%s:%d", configuration.MQTTHost, configuration.MQTTPort),
				Options: []mqtt.DialOption{
					mqtt.WithConnStateHandler(func(s mqtt.ConnState, err error) {
						// Register ConnState callback to low level client
						klog.V(3).Infof("State changed to %s (err: %v)", s, err)
					}),
				},
			},
			mqtt.WithPingInterval(10*time.Second),
			mqtt.WithTimeout(5*time.Second),
			mqtt.WithReconnectWait(1*time.Second, 15*time.Second),
		)
		if err != nil {
			klog.Errorf("%v", err)
			os.Exit(1)
		}

		//
		go func() {
			// Establish the first connection to the MQTT broker. A broker that is
			// not reachable yet is not fatal - keep retrying with backoff so the
			// bridge comes up on its own once the network is back.
			connectWait := retryInitialWait
			for {
				_, connErr := MQTTClient.Connect(BaseContext,
					"DigitalstromBridge", // Client ID
					mqtt.WithKeepAlive(30),
					mqtt.WithWill(
						&mqtt.Message{
							Topic:   "tele/dssBridge/LWT",
							QoS:     mqtt.QoS1,
							Payload: []byte("Offline"),
							Retain:  true,
						},
					))
				if connErr == nil {
					break
				}
				if BaseContext.Err() != nil {
					return
				}
				klog.Warningf("Unable to connect to MQTT server: %v (retrying in %s)", connErr, connectWait)
				connectWait = retryWait(BaseContext, connectWait)
			}

			// Publish on Last Will Topic that we are Online
			if err := MQTTClient.Publish(BaseContext, &mqtt.Message{
				Topic:   "tele/dssBridge/LWT",
				QoS:     mqtt.QoS1,
				Payload: []byte("Online"),
				Retain:  true,
			}); err != nil {
				klog.Warningf("MQTTClient.Publish (LWT Online) failed: %v", err)
			}

			mux := &mqtt.ServeMux{} // Multiplex message handlers by topic name.
			MQTTClient.Handle(mux)  // Register mux as a low-level handler.

			var AuthenticatedContext *context.Context = new(context.Context)

			var validGroupID = regexp.MustCompile(`^cmnd/dssBridge/group/([0-9]+)/([0-9]+)$`)

			mux.Handle("cmnd/dssBridge/group/+/+", // Handle all topics by this handler.
				mqtt.HandlerFunc(func(msg *mqtt.Message) {
					klog.V(3).Infof("Message '%s': '%s' (QoS: %d)", msg.Topic, []byte(msg.Payload), int(msg.QoS))
					sm := validGroupID.FindAllStringSubmatch(msg.Topic, -1)
					if len(sm) == 1 {
						ZoneID, _ := strconv.ParseInt(sm[0][1], 0, 32)
						GroupID, _ := strconv.ParseInt(sm[0][2], 0, 32)
						SceneID, _ := strconv.ParseInt(string(msg.Payload), 0, 32)

						ac.ZoneApi.ZoneCallScene(
							*AuthenticatedContext,
							int32(SceneID),
							&swagger.ZoneApiZoneCallSceneOpts{
								GroupID: optional.NewInt32(int32(GroupID)),
								Id:      optional.NewInt32(int32(ZoneID))})
					}
				}),
			)

			// Subscribe to the command topic. Retry with backoff instead of
			// terminating if the broker is temporarily unavailable.
			subscribeWait := retryInitialWait
			for {
				_, subErr := MQTTClient.Subscribe(BaseContext,
					mqtt.Subscription{
						Topic: "cmnd/dssBridge/#",
						QoS:   mqtt.QoS1,
					},
				)
				if subErr == nil {
					break
				}
				if BaseContext.Err() != nil {
					return
				}
				klog.Warningf("Unable to subscribe to command topic: %v (retrying in %s)", subErr, subscribeWait)
				subscribeWait = retryWait(BaseContext, subscribeWait)
			}

			loginWait := retryInitialWait
			for BaseContext.Err() == nil {
				b1, _, b3 := ac.AuthenticationApi.Login(BaseContext, configuration.Username, configuration.Password)

				if b3 != nil || !b1.Ok || b1.Result == nil {
					// A failed login is most commonly caused by the dSS being
					// unreachable. Do not give up - wait and try again.
					klog.Warningf("Authentication for user '%s' on dSS '%s' failed: %v (retrying in %s)",
						configuration.Username, configuration.BaseURL, b3, loginWait)
					loginWait = retryWait(BaseContext, loginWait)
					continue
				}
				klog.V(3).Infof("Authenticated on dSS '%s'", configuration.BaseURL)

				// Build a context that carries the freshly obtained session token
				// and is cancelled as soon as this session ends (token expiry or
				// connectivity loss), so goroutines tied to it stop as well.
				SessionContext, endSession := context.WithCancel(BaseContext)
				*AuthenticatedContext = context.WithValue(
					SessionContext,
					swagger.ContextAPIKey,
					swagger.APIKey{Key: b1.Result.Token})

				var SubscriptionID int32 = 4711

				go publishHomeassistantAdvertisments(
					*AuthenticatedContext,
					*ac,
					MQTTClient)

				if _, _, err := ac.EventApi.Subscribe(*AuthenticatedContext, "callScene", SubscriptionID); err != nil {
					klog.Warningf("Event subscription 'callScene' failed: %v", err)
				}
				if _, _, err := ac.EventApi.Subscribe(*AuthenticatedContext, "buttonClick", SubscriptionID); err != nil {
					klog.Warningf("Event subscription 'buttonClick' failed: %v", err)
				}

				// Event receive loop for the current session. After a number of
				// consecutive failures the session is dropped and rebuilt (fresh
				// login and subscription).
				eventFailures := 0
				eventWait := retryInitialWait
				sessionHadSuccess := false
				for BaseContext.Err() == nil {
					ev, _, err := ac.EventApi.Get(
						*AuthenticatedContext,
						SubscriptionID,
						&swagger.EventApiGetOpts{
							Timeout: optional.NewInt32(
								int32(messageReceiveTimeout.Milliseconds()) - 2000)})

					if err != nil {
						eventFailures++
						klog.Warningf("Receiving events from dSS failed (attempt %d/%d): %v",
							eventFailures, consecutiveEventFailures, err)
						if eventFailures >= consecutiveEventFailures {
							break
						}
						eventWait = retryWait(BaseContext, eventWait)
						continue
					}
					if !ev.Ok || ev.Result == nil {
						klog.Warningf("dSS reported the event subscription as invalid - re-authenticating")
						break
					}

					// Successful poll - reset the failure tracking.
					eventFailures = 0
					eventWait = retryInitialWait
					sessionHadSuccess = true

					if len(ev.Result.Events) < 1 {
						continue
					}

					if debugmode {
						spew.Dump(ev)
					}

					for _, v := range ev.Result.Events {
						Topic := "stat/dssBridge/"
						Payload := ""
						Retain := false
						switch v.Name {
						case "callScene":
							Topic = Topic + "group/" + fmt.Sprintf("%s/%s", v.Properties.ZoneID, v.Properties.GroupID)
							Payload = fmt.Sprintf("{\"scene\": \"%s\"}", v.Properties.SceneID)
							Retain = true
						case "buttonClick":
							Topic = Topic + "switch/" + v.Source.Dsid
							Payload = fmt.Sprintf("{\"buttonIndex\": %s, \"clickType\": %s}", v.Properties.ButtonIndex, v.Properties.ClickType)
						}
						klog.V(2).Infof("Publishing Topic '%s' Payload '%s'", Topic, Payload)
						if err := MQTTClient.Publish(BaseContext, &mqtt.Message{
							Topic:   Topic,
							QoS:     mqtt.QoS1,
							Payload: []byte(Payload),
							Retain:  Retain,
						}); err != nil {
							klog.Warningf("MQTTClient.Publish - Error: %v", err)
						}
					}
				}

				// The session ended (connectivity loss or invalid token). Cancel
				// the session context so the discovery goroutine stops, then loop
				// around to authenticate again.
				endSession()

				if sessionHadSuccess {
					// A healthy session just dropped - reconnect promptly.
					loginWait = retryInitialWait
				} else {
					// We could authenticate but never received a single event;
					// something is still wrong. Back off before trying again to
					// avoid a tight re-login loop.
					klog.Warningf("dSS session ended without receiving any event (retrying in %s)", loginWait)
					loginWait = retryWait(BaseContext, loginWait)
				}

				//_, _, _ = ac.AuthenticationApi.Logout(AuthenticatedContext)
				//

				//break
			}
			//if BaseContext.Err() != context.Canceled {
			//	if err := MQTTClient.Disconnect(BaseContext); err != nil {
			//		fmt.Printf("Error: %v\n", err)
			//		os.Exit(1)
			//	}
			//}
			//log.Print("Eventloop ended")
			//
			//InnerDone <- true
		}()

		<-InnerDone

		//klog.Info("Inner Done")

		//if cc3 != nil {
		//	dd1, _, dd3 := ac.DeviceApi.TurnOn(
		//		ctx,
		//		"303505d7f80000400001e053")
		//	if dd3 == nil && dd1.Ok {
		//	}
		//
		//	time.Sleep(time.Second * 3)
		//
		//	dd1, _, dd3 = ac.DeviceApi.TurnOff(
		//		ctx,
		//		"303505d7f80000400001e053")
		//	if dd3 == nil && dd1.Ok {
		//	}
		//}

	}()

	<-BaseContext.Done()

	// /json/apartment/getStructure
	// Listet die Meter => /json/property/query?query=/apartment/dSMeters/*(dSID,powerConsumption,energyMeterValue,dSUID)
	// Stromverbrauch aller Meter => /json/property/query?query=/apartment/dSMeters/*(dSID,powerConsumption,energyMeterValue,dSUID)

	klog.Info("Server ended")
}
