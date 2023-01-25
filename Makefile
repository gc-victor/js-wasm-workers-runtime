ARGUMENTS = $(filter-out $@,$(MAKECMDGOALS))

ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
WASI_SDK ?= ${ROOT_DIR}/crates/engine/wasi-sdk
WASI_VERSION ?= 17
WASI_VERSION_FULL ?= ${WASI_VERSION}.0

build: install_wasi_sdk pnpm release

pnpm:
	make -C crates/engine pnpm

esbuild:
	make -C crates/engine esbuild 

clean:
	make -C crates/engine clean

run:
	make -C crates/engine run

debug: esbuild
	make -C crates/engine debug

release: esbuild
	make -C crates/engine release

# Run the examples
example:
	cargo run --example $(ARGUMENTS) --manifest-path examples/Cargo.toml --release

# Install wasi-sdk
install_wasi_sdk:
	make -C crates/engine install_wasi_sdk

improve:
	# --all-target: apply clippy to all targets
	# --all-features: check all available features
	# --workspace: check all packages in a workspace
	QUICKJS_WASM_SYS_WASI_SDK_PATH=$(WASI_SDK) \
	cargo clippy --all-targets --all-features --workspace -- -D warnings
	cargo fmt --all -- --check

test-engine:
	QUICKJS_WASM_SYS_WASI_SDK_PATH=$(WASI_SDK) \
	cargo wasi test --package js-wasm-workers-engine -- --nocapture 

# catch anything and do nothing
%:
	@: