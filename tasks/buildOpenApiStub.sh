#!/bin/bash

# Variables
IMAGE_NAME="swaggerapi/swagger-codegen-cli"
CONTAINER_NAME="openapi_stub_generator"
OPENAPI_SPEC="./assets/swagger/digitalStrom.yaml"
OUTPUT_DIR="./internal/pkg/swagger/"

# Run Docker container to generate stub from OpenAPI spec
docker run --rm \
    --name $CONTAINER_NAME \
    -v "$(pwd):/local" \
    $IMAGE_NAME generate \
    -i /local/$OPENAPI_SPEC \
    -l go \
    -o /local/$OUTPUT_DIR