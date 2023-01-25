ARGUMENTS = $(filter-out $@,$(MAKECMDGOALS))

ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
WASI_SDK ?= ${ROOT_DIR}/wasi-sdk
WASI_VERSION ?= 17
WASI_VERSION_FULL ?= ${WASI_VERSION}.0

export QUICKJS_WASM_SYS_WASI_SDK_PATH=$(WASI_SDK)

build: install_wasi_sdk pnpm release

pnpm:
	pnpm install

esbuild:
	./node_modules/.bin/esbuild --bundle --format=iife --minify --tree-shaking=false --outfile=dist/web-platform-apis.js src/web-platform-apis/index.js

clean:
	cargo clean
	rm Cargo.lock

run:
	cargo run --target wasm32-wasi

debug: esbuild
	cargo build --target wasm32-wasi

release: esbuild
	cargo build --target wasm32-wasi --release

# Run the examples
example:
	cargo run --example $(ARGUMENTS) --manifest-path examples/Cargo.toml --release

# Install wasi-sdk
install_wasi_sdk:
	@echo "Installing WASI SDK..."
	[ -d ./wasi-sdk ] && \
	echo "wasi_sdk is already installed." && \
	exit || \
	wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_VERSION}/wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz && \
	tar xvf wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz && \
	mv wasi-sdk-${WASI_VERSION_FULL} wasi-sdk && \
	rm wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz \

improve:
	# --all-target: apply clippy to all targets
	# --all-features: check all available features
	# --workspace: check all packages in a workspace
	cargo clippy --all-targets --all-features --workspace -- -D warnings
	cargo fmt --all -- --check

# catch anything and do nothing
%:
	@: