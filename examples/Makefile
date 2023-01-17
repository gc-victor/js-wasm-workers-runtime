ARGUMENTS = $(filter-out $@,$(MAKECMDGOALS))

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

example:
	cargo run --example $(ARGUMENTS)

# catch anything and do nothing
%:
	@: