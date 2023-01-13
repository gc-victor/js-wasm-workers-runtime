ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
WASI_SDK ?= ${ROOT_DIR}/wasi-sdk
WASI_VERSION ?= 17
WASI_VERSION_FULL ?= ${WASI_VERSION}.0

build: install_wasi_sdk pnpm release

pnpm:
	pnpm install

esbuild:
	./node_modules/.bin/esbuild --bundle --format=iife --minify --tree-shaking=false --outfile=dist/web-platform-apis.js src/web-platform-apis/index.js

run:
	QUICKJS_WASM_SYS_WASI_SDK_PATH=$(WASI_SDK) \
	cargo run --target wasm32-wasi

debug: esbuild
	QUICKJS_WASM_SYS_WASI_SDK_PATH=$(WASI_SDK) \
	cargo build --target wasm32-wasi

release: esbuild
	QUICKJS_WASM_SYS_WASI_SDK_PATH=$(WASI_SDK) \
	cargo build --target wasm32-wasi --release

example_logger:
	cargo run --example logger --manifest-path examples/Cargo.toml

example_runtime:
	cargo run --example runtime --manifest-path examples/Cargo.toml

install_wasi_sdk:
	@echo "Installing WASI SDK..."
	[ -d ./wasi-sdk ] && \
	echo "wasi_sdk is already installed." && \
	exit || \
	wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_VERSION}/wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz && \
	tar xvf wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz && \
	mv wasi-sdk-${WASI_VERSION_FULL} wasi-sdk && \
	rm wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz \
