# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A Go bridge between a digitalStrom smart-home server (dSS) and MQTT. It logs into the dSS JSON API, subscribes to
its event stream (`callScene`, `buttonClick`), republishes those events to MQTT topics, publishes Home Assistant
MQTT-discovery messages for zones/groups, and listens on an MQTT command topic to trigger scenes back on the dSS.

## Generated Swagger client (read this before building)

`internal/pkg/swagger/` is **generated code and is gitignored** (`git ls-files` shows it untracked) — it does not
exist in a fresh checkout and must be generated before `go build`/`go vet`/anything else will compile.

Generate it with (requires Docker):
```bash
go generate ./...              # runs the //go:generate directive in cmd/DigitalstromMQTTBridge/main.go
./tasks/buildOpenApiStub.sh    # same thing, invoked directly; cwd-independent
```
or the equivalent manual command (also used by CI, see `.github/workflows/go.yml` and `build/docker/Dockerfile`):
```bash
docker run --rm --user $(id -u):$(id -g) -v "$(pwd):/local" swaggerapi/swagger-codegen-cli generate \
  -i /local/assets/swagger/digitalStrom.yaml -l go -o /local/internal/pkg/swagger/
```
The source of truth for the client is `assets/swagger/digitalStrom.yaml` (a Swagger 2.0 spec for the dSS JSON API).
To change the generated API surface, edit that spec and regenerate — do not hand-edit files under
`internal/pkg/swagger/`.

Note: `go build`/`go vet` do **not** trigger `go generate` automatically (that's a Go language decision, not
something specific to this repo) — either run `go generate ./...` first, or use the `Makefile` targets below, which
run it for you.

## Build / run

```bash
make build                              # go generate ./... + go build ./...
make vet                                # go generate ./... + go vet ./...
make run ARGS="-baseURL https://<dss-host>:8080 -username dssadmin -password <pw> -mqtthost <host> -mqttport 1883"
make clean                              # removes internal/pkg/swagger/ and bin/
```
or drive it manually:
```bash
go generate ./...                       # regenerates internal/pkg/swagger/ (requires Docker, see above)
go build ./...
go vet ./...
go run ./cmd/DigitalstromMQTTBridge -baseURL https://<dss-host>:8080 -username dssadmin -password <pw> -mqtthost <host> -mqttport 1883
```
There are no Go test files in this repo currently.

Go toolchain: `go.mod` requires Go >= 1.25.

Docker image build (runs the codegen step itself, no local Docker/pre-generation needed beforehand):
```bash
docker build -t digitalstrommqttbridge -f build/docker/Dockerfile .
```

Updating dependencies:
```bash
go get -u
go mod tidy
```

## Architecture

- `cmd/DigitalstromMQTTBridge/main.go` — parses CLI flags (`-baseURL`, `-username`, `-password`, `-mqtthost`,
  `-mqttport`) into a `dssinterface.DssBridgeConfiguration` and calls `StartDssBridge`. All real logic lives in the
  `internal/pkg/dSSInterface` package (Go package name `dssinterface`).
- `internal/pkg/dSSInterface/dSSInterface.go` — the entire bridge. Key flow inside `StartDssBridge`:
  1. Builds an `http.Client` with `InsecureSkipVerify: true` (dSS boxes typically use self-signed certs) and wraps
     it in a generated `swagger.APIClient`.
  2. Connects to MQTT (`mqtt-go`), publishes an Online/Offline LWT on `tele/dssBridge/LWT`, and subscribes to
     `cmnd/dssBridge/#`. Messages on `cmnd/dssBridge/group/<zoneId>/<groupId>` (payload = scene id) call
     `ZoneApi.ZoneCallScene` on the dSS.
  3. Logs into the dSS (`AuthenticationApi.Login`), wraps the resulting session token into the request context via
     `swagger.ContextAPIKey` — this `AuthenticatedContext` is what every subsequent dSS API call must use.
  4. Subscribes to dSS events `callScene` and `buttonClick` (`EventApi.Subscribe`) under a fixed
     `SubscriptionID`, then long-polls `EventApi.Get` in a loop, translating events to MQTT publishes:
     - `callScene` → `stat/dssBridge/group/<zoneId>/<groupId>` with `{"scene": "<id>"}` (retained)
     - `buttonClick` → `stat/dssBridge/switch/<dsid>` with `{"buttonIndex": .., "clickType": ..}`
  5. `publishHomeassistantAdvertisments` runs concurrently per authenticated session, polling the apartment
     structure (`ApartmentApi.GetStructure`) once a minute and publishing Home Assistant MQTT-discovery configs
     (`homeassistant/switch/dssBridge/<zoneId>_<applicationType>/config`) for zones/groups with `ApplicationType == 1`
     (lights).
  - **Resilience model**: every network operation (MQTT connect/subscribe, dSS login, event polling) retries
    indefinitely with exponential backoff (`retryInitialWait`/`retryMaxWait`/`retryWait`) instead of exiting the
    process — the process is expected to run unattended and self-heal through dSS/MQTT/network outages. A dSS
    session is torn down and re-authenticated after `consecutiveEventFailures` failed event polls in a row, or when
    the dSS reports the subscription as invalid.
  - `SIGINT`/`SIGTERM` cancel a single root `context.Context` (`BaseContext`) that everything is derived from, which
    unwinds the retry loops and goroutines.
- `internal/pkg/dSSInterface/dSSStructures.go` — a couple of small hand-written request/response structs not covered
  by the generated client.
- `internal/pkg/swagger/` — generated client for the dSS JSON API (see above). Treat as vendored/generated; don't
  hand-edit.
- `assets/swagger/digitalStrom.yaml` — the OpenAPI/Swagger 2.0 spec that the client is generated from; this is the
  place to add/change dSS API coverage.

## dSS domain notes (from README)

Useful vocabulary when reading events/topics or the swagger spec:
- Zone = "Raum" (room), Group = functional group within a zone, Cluster = "Gruppen".
- Scene IDs: `0` = Off, `5` = On ("Stimmung1"), `17`/`18`/`19` = Stimmung2/3/4.
- Click types: `0` = single click, `1` = double click, `6` = long press (>5s).
- `applicationType` on a group (color-coded in the dSS UI): `1` = Lights (yellow), `2` = Blinds (gray),
  `3` = Climate (blue), `4` = Audio (cyan), `5` = Video (magenta), Security = red, Access = green, `8` = Black,
  `9` = Cooling, `10` = Ventilation, `11` = Window, `12` = Recirculation, `48` = control temperature.
- MQTT topic shape: `stat/dssBridge/group/<zoneId>/<groupId>` (state) and `cmnd/dssBridge/group/<zoneId>/<groupId>`
  (command, payload = scene id), plus `stat/dssBridge/switch/<dSID>` for button events.
- To capture/inspect raw dSS JSON traffic (e.g. from the official app) the README documents a Burp Suite proxy setup
  pointed at the dSS host on port 8080 with SSL interception forced.
