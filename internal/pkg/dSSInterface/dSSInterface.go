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

	"k8s.io/klog/v2"
)

const messageReceiveTimeout = 120 * time.Second
const debugmode = false

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
		c1, _, _ := ac.ApartmentApi.GetStructure(ctx)

		for _, v := range c1.Result.Apartment.Zones {
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
				// Publish on Last Will Topic that we are Online
				if err := MQTTClient.Publish(ctx, &mqtt.Message{
					Topic:   fmt.Sprintf("homeassistant/switch/dssBridge/%d_%d/config", v.Id, g.ApplicationType),
					QoS:     mqtt.QoS1,
					Payload: pl,
					Retain:  false,
				}); err != nil {
					klog.Exitf("MQTTClient.Publish - Error: %v", err)
				}

				klog.V(3).Infof("Published Homeassistant discovery info for Zone %d: %s", v.Id, v.Name)
			}
		}
		time.Sleep(time.Minute)
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
			_, err = MQTTClient.Connect(BaseContext,
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
			if err != nil {
				klog.Errorf("%v", err)
				os.Exit(1)
			}

			// Publish on Last Will Topic that we are Online
			if err := MQTTClient.Publish(BaseContext, &mqtt.Message{
				Topic:   "tele/dssBridge/LWT",
				QoS:     mqtt.QoS1,
				Payload: []byte("Online"),
				Retain:  true,
			}); err != nil {
				klog.Exitf("MQTTClient.Publish - Error: %v", err)
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

			// Subscribe two topics.
			if err := MQTTClient.Subscribe(BaseContext,
				mqtt.Subscription{
					Topic: "cmnd/dssBridge/#",
					QoS:   mqtt.QoS1,
				},
			); err != nil {
				klog.Exitf("Error: %v", err)
			}

			for {
				b1, _, b3 := ac.AuthenticationApi.Login(BaseContext, configuration.Username, configuration.Password)

				if b3 != nil {
					klog.V(3).Infof("Authentication result=%s (%s)", b1.Result, b3)
				}
				if !b1.Ok {
					klog.Exitf("Authentication request for user '%s' on dSS '%s' failed", configuration.Username, configuration.BaseURL)
					cancel()
				}
				// Insert authentication Data into context
				*AuthenticatedContext = context.WithValue(
					BaseContext,
					swagger.ContextAPIKey,
					swagger.APIKey{Key: b1.Result.Token})

				var SubscriptionID int32 = 4711

				go publishHomeassistantAdvertisments(
					*AuthenticatedContext,
					*ac,
					MQTTClient)

				_, _, _ = ac.EventApi.Subscribe(*AuthenticatedContext, "callScene", SubscriptionID)
				_, _, _ = ac.EventApi.Subscribe(*AuthenticatedContext, "buttonClick", SubscriptionID)

				for {
					ev, _, err := ac.EventApi.Get(
						*AuthenticatedContext,
						SubscriptionID,
						&swagger.EventApiGetOpts{
							Timeout: optional.NewInt32(
								int32(messageReceiveTimeout.Milliseconds()) - 2000)})

					if err != nil || !ev.Ok || len(ev.Result.Events) < 1 {
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
							Topic = Topic + "switch/" + fmt.Sprintf("%s", v.Source.Dsid)
							Payload = fmt.Sprintf("{\"buttonIndex\": %s, \"clickType\": %s}", v.Properties.ButtonIndex, v.Properties.ClickType)
						}
						klog.V(2).Infof("Publishing Topic '%s' Payload '%s'", Topic, Payload)
						if err := MQTTClient.Publish(BaseContext, &mqtt.Message{
							Topic:   Topic,
							QoS:     mqtt.QoS1,
							Payload: []byte(Payload),
							Retain:  Retain,
						}); err != nil {
							klog.Exitf("MQTTClient.Publish - Error: %v", err)
						}
					}
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
