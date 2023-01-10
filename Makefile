ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
WASI_SDK ?= ${ROOT_DIR}/wasi-sdk
WASI_VERSION ?= 14
WASI_VERSION_FULL ?= ${WASI_VERSION}.0

run:
	QUICKJS_WASM_SYS_WASI_SDK_PATH=$(WASI_SDK) \
	cargo run --target wasm32-wasi

build:
	QUICKJS_WASM_SYS_WASI_SDK_PATH=$(WASI_SDK) \
	cargo build --target wasm32-wasi --release

install_wasi_sdk:
	@echo "Installing WASI SDK..."
	export WASI_VERSION=14 && \
	export WASI_VERSION_FULL=${WASI_VERSION}.0 && \
	wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_VERSION}/wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz && \
	tar xvf wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz && \
	mv wasi-sdk-${WASI_VERSION_FULL} _wasi-sdk && \
	rm wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz