#!/bin/bash

set +x
echo "Generating OpenAPI stub..."
echo "========================"

# Variables
IMAGE_NAME=swaggerapi/swagger-codegen-cli
CONTAINER_NAME=openapi_stub_generator
OPENAPI_SPEC=assets/swagger/digitalStrom.yaml
OUTPUT_DIR=internal/pkg/swagger/

rm -rf $OUTPUT_DIR
mkdir -p $OUTPUT_DIR

# Run Docker container to generate stub from OpenAPI spec
docker run --rm \
    --name $CONTAINER_NAME \
    --user $(id -u):$(id -g) \
    -v "$(pwd):/local" \
    $IMAGE_NAME generate \
    -i /local/$OPENAPI_SPEC \
    -l go \
    -o /local/$OUTPUT_DIR
