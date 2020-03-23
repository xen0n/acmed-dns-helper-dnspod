TOP := $(shell pwd)

all: help

.PHONY: rel
rel:
	mkdir -p "${TOP}/target" "${TOP}/ci-build/registry-cache" "${TOP}/ci-build/target"
	docker run --rm -it \
		-v "${TOP}":/home/rust/src:ro \
		-v "${TOP}/ci-build/registry-cache":/home/rust/.cargo/registry \
		-v "${TOP}/ci-build/target":/home/rust/src/target \
		ekidd/rust-musl-builder:1.42.0-openssl11 cargo build --release

.PHONY: help
help:
	@echo usage:
	@echo
	@echo make rel  - make release binary with docker
