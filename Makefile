.PHONY: all 
all: check build-linux build-windows build-macos build-macos-intel

REPO ?= 2030047311
TAG ?= latest

check:
	cargo check

build:
	RUSTFLAGS='-C link-arg=-s' cargo build --release

build-linux:
	cargo build --release --target=x86_64-unknown-linux-gnu 
	cargo build --release --target=aarch64-unknown-linux-gnu

build-windows:
	cargo build --release --target=x86_64-pc-windows-gnu

build-macos:
	cargo build --release --target=aarch64-apple-darwin

build-macos-intel:
	cargo build --release --target=x86_64-apple-darwin

build-image:
	docker build -f Dockerfile -t ${REPO}/kube-robot:${TAG} .
	docker push ${REPO}/kube-robot:${TAG}

build-multi-platform-image:
	docker buildx build -f Dockerfile -t ${REPO}/kube-robot:${TAG} --platform linux/amd64,linux/arm64 --push .

clean:
	cargo clean

help:
	@echo "usage: make build-linux or make build-windows or make build-macos"
