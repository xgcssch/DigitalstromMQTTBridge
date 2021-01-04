# Build with
# docker build -t digitalstrommqttbridge -f build/docker/Dockerfile .
# from root directory

FROM swaggerapi/swagger-codegen-cli as generator
COPY ./assets/swagger/digitalStrom.yaml /local/input/

RUN mkdir /local/output/ && \
    java -jar /opt/swagger-codegen-cli/swagger-codegen-cli.jar generate -i "/local/input/digitalStrom.yaml" -l go -o "/local/output/"

FROM golang:alpine as builder

WORKDIR /go/app
COPY . .
COPY --from=generator /local/output/ ./internal/pkg/swagger/

RUN mkdir bin && \
    GOOS=linux go build -v -o bin ./... && \
    go install -v ./...

FROM alpine:latest

ENV BASEURL=https://dss.int.acme.com:8080 \
    USERNAME=dssadmin \
    PASSWORD=unknown \
    MQTTHOST=mqtt.int.acme.com \
    MQTTPORT=1883 \
    VERBOSITY=1

RUN apk --no-cache add ca-certificates
WORKDIR /root/
COPY --from=builder /go/app/bin/DigitalstromMQTTBridge .

CMD ./DigitalstromMQTTBridge -baseURL "$BASEURL" -username "$USERNAME" -password "$PASSWORD" -mqtthost "$MQTTHOST" -mqttport "$MQTTPORT" -v "$VERBOSITY"
