//
// Main entry for the SystaComfort Prometheus exporter
//

package main

import (
	"flag"

	internal "github.com/xgcssch/DigitalstromMQTTBridge/internal/pkg/dSSInterface"
	"k8s.io/klog/v2"
)

var baseURL = flag.String("baseURL", "nil", "Base URL of dSS")
var username = flag.String("username", "dssadmin", "Username of dSS")
var password = flag.String("password", "nil", "Password")
var mqttHost = flag.String("mqtthost", "nil", "Hostname of the MQTT Server")
var mqttPort = flag.Int("mqttport", 1883, "Port of the MQTT Server")

func main() {
	klog.InitFlags(nil)
	flag.Parse()

	klog.Info("DigitalstromBridge v0.1 starting")

	Configuration := internal.DssBridgeConfiguration{
		BaseURL:  *baseURL,
		Username: *username,
		Password: *password,
		MQTTHost: *mqttHost,
		MQTTPort: *mqttPort,
	}

	internal.StartDssBridge(Configuration)
}
