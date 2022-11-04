# DigitalstromMQTTBridge
Bridge Digitalstrom devices to MQTT and generate Data for Prometheus
# dSS JsonApi
```
docker run --rm -p 80:8080 swaggerapi/swagger-editor
```
## Generate OpenApi Rust client
```
$ cd DigitalstromMQTTBridge
docker run --rm \
  -v "$(pwd):/local"  \
  --user $(id -u):$(id -g)  \
  openapitools/openapi-generator-cli generate \
    -i /local/assets/openapi/digitalStrom.yaml \
    -g rust \
    -o /local/dss_openapi \
    -p packageName=dss_openapi
```
# Debug with Burp Suite
To explore the JSON Communication between the MQTTBridge or any other Program using this interface (e. g. Digitalstrom App) and the dSS you can use the [[Burp Suite]](asdf)
- Start Burp
- Create temporary project -> Next
- Use Burp defaults -> Start Burp
- Select Proxy tab
- Select Option nested tab
- Edit existing listener entry for "127.0.0.01:8080"
- Select "Binding" tab
- Change "Bind to address" to the "All interfaces" selection
- Select "Request handling" tab
- Enter your dSS Hostname in the "Redirect to host" field
- Enter "8080" in the "Redirect to port" field
- Select "Force use of SSL" option
- Select "Support invisible proxying" option
# dSS
ZoneId=Raum
groupId=
cluster=Gruppen

SceneId
0=Off
5=On (Stimmung1)
17=Stimmung2
18=Stimmung3
19=Stimmung4

clickType=0 => einmal
clickType=1 => zweimal
clickType=6 => länger als 5s

applicationType 
Color=0=broadcast

Room lights, Garden lights, Building illuminations          => Yellow = 1 = Lights
Blinds, Shades, Awnings, Curtains                           => Gray = 2 = Blinds
Heating, Cooling, Ventilation, Temperature control, Window  => Blue = 3 = Climate
Music, Radio                                                => Cyan = 4 = Audio
TV, Video                                                   => Magenta = 5 = Video
Alarms, Fire, Panic                                         => Red = Security
Doors, Door bells, Access control                           => Green = Access

Black = 8 
Cooling = 9
Ventilation = 10 
Window = 11
recirculation = 12
controltemperature = 48
# MQTT
https://mqtt-explorer.com/
stat/dssBridge/group/3/1 -> zoneID=3 groupId=1
cmnd/dssBridge/group/<ZoneId>/<GroupId> <SceneId>
cmnd/dssBridge/group/3/1 5

https://homieiot.github.io/

https://www.home-assistant.io/docs/mqtt/discovery/
=>

{
  "name": "Gosund-03",
  "stat_t": "tele/Gosund-03/STATE",
  "avty_t": "tele/Gosund-03/LWT",
  "pl_avail": "Online",
  "pl_not_avail": "Offline",
  "cmd_t": "cmnd/Gosund-03/POWER",
  "val_tpl": "{{value_json.POWER}}",
  "pl_off": "OFF",
  "pl_on": "ON",
  "uniq_id": "BE5891_RL_1",
  "dev": {
    "ids": [
      "BE5891"
    ]
  }
}

cmnd/Gosund-03/POWER
ON

tele/Gosund-03/STATE
{
  "Time": "2020-12-20T16:56:31",
  "Uptime": "10T02:55:16",
  "UptimeSec": 874516,
  "Heap": 28,
  "SleepMode": "Dynamic",
  "Sleep": 50,
  "LoadAvg": 19,
  "MqttCount": 1,
  "POWER": "OFF",
  "Wifi": {
    "AP": 1,
    "SSId": "lykke_2.4_nomap",
    "BSSId": "6E:3B:6B:3E:B4:10",
    "Channel": 9,
    "RSSI": 66,
    "Signal": -67,
    "LinkCount": 1,
    "Downtime": "0T00:00:06"
  }
}


https://www.home-assistant.io/integrations/switch.mqtt/

homeassistant/switch/dssBridge/3/config

{
  "name": "Arbeitszimmer",
  "stat_t": "stat/dssBridge/group/3/1",
  "avty_t": "tele/dssBridge/LWT",
  "pl_avail": "Online",
  "pl_not_avail": "Offline",
  "cmd_t": "cmnd/dssBridge/group/3/1",
  "val_tpl": "{{value_json.scene}}",
  "pl_off": "0",
  "pl_on": "5",
  "stat_off": "0",
  "stat_on": "5",
  "uniq_id": "dssBridge_3",
  "dev": {
    "ids": [
      "dssBridge_3"
    ]
  }
}

https://www.home-assistant.io/integrations/device_trigger.mqtt/

homeassistant/device_automation/dssBridge/303505d7f8000000000016800001f2d200_config

{
  "automation_type": "trigger",
  "topic": "stat/dssBridge/switch/303505d7f8000000000016800001f2d200",
  "type": "button_short_press",
  "subtype": "button_1",
  "device": {
    "identifiers": [
      "303505d7f8000000000016800001f2d200"
    ],
    "manufacturer": "digitalStrom",
    "model": "SW-TKM200"
    "name": "Schalter Arbeitszimmer rechts-oben"
  }
}

homeassistant/device_automation/dssBridge/303505d7f8000000000016800001d06f00/config

{
  "automation_type": "trigger",
  "topic": "stat/dssBridge/switch/303505d7f8000000000016800001d06f00",
  "type": "button_short_press",
  "subtype": "button_1",
  "device": {
    "identifiers": [
      "303505d7f8000000000016800001d06f00"
    ],
    "manufacturer": "digitalStrom",
    "model": "SW-TKM200"
    "name": "Eingang links - unten - unten - rechts"
  }
}

