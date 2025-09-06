FROM swaggerapi/swagger-codegen-cli AS generator
COPY assets/swagger/digitalStrom.yaml /local/input/

RUN mkdir /local/output/ && \
    java -jar /opt/swagger-codegen-cli/swagger-codegen-cli.jar generate -i "/local/input/digitalStrom.yaml" -l go -o "/local/output/"

FROM mcr.microsoft.com/devcontainers/go:1-1.24-bookworm AS builder

WORKDIR /go/app
COPY . .
COPY --from=generator /local/output/ ./internal/pkg/swagger/

RUN ls ./internal/pkg/swagger/ && mkdir bin
