ARGUMENTS = $(filter-out $@,$(MAKECMDGOALS))

ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
WASI_SDK ?= ${ROOT_DIR}/crates/engine/wasi-sdk
WASI_VERSION ?= 17
WASI_VERSION_FULL ?= ${WASI_VERSION}.0

build: engine-install-wasi-sdk engine-pnpm engine-release

engine-pnpm:
	make -C crates/engine pnpm

engine-esbuild:
	make -C crates/engine esbuild 

clean:
	cargo clean
	rm Cargo.lock

engine-run:
	make -C crates/engine run

engine-debug: esbuild
	make -C crates/engine debug

engine-release: esbuild
	make -C crates/engine release

engine-install-wasi-sdk:
	make -C crates/engine install-wasi-sdk

example: release
	cargo run --example $(ARGUMENTS) --manifest-path examples/Cargo.toml

improve:
	QUICKJS_WASM_SYS_WASI_SDK_PATH=$(WASI_SDK) \
	cargo clippy --all-targets --all-features --workspace
	cargo fmt --all -- --check

test-engine:
	QUICKJS_WASM_SYS_WASI_SDK_PATH=$(WASI_SDK) \
	cargo wasi test --package js-wasm-workers-engine -- --nocapture

rome-check:
	crates/engine/node_modules/.bin/rome check crates/engine/web-platform-apis --apply


# catch anything and do nothing
%:
	@: