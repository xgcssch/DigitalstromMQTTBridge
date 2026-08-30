.PHONY: generate build vet run clean

# Regenerate the dSS swagger client (internal/pkg/swagger/) from
# assets/swagger/digitalStrom.yaml. Requires Docker.
generate:
	go generate ./...

# Regenerate the client (if missing) and build the binary.
build: generate
	go build ./...

vet: generate
	go vet ./...

run: generate
	go run ./cmd/DigitalstromMQTTBridge $(ARGS)

clean:
	rm -rf internal/pkg/swagger bin
