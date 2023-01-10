# Choose a JS Engine

* Status: accepted
- Deciders: [@gc-victor](https://github.com/gc-victor)

Technical Story: Choose a JS Engine for the runtime

## Context and Problem Statement

It is necessary to choose a JS Engine to be used in the runtime. There are several options available, and it is required to choose one for our use case.

We need to consider the following aspects:

- Performance
- Memory usage
- Size of the binary
- Compatibility with the WASI API
- Compatibility with the Rust ecosystem
- Ease of use
- Documentation
- Community support
- License
- Security
- Stability

## Decision Drivers

* [@gc-victor](https://github.com/gc-victor)

## Considered Options

* [V8 (Rusty V8 Binding)](https://crates.io/crates/v8)
* [QuickJS (quickjs-wasm-rs)](https://crates.io/crates/quickjs-wasm-rs)

Other options in the market:

* SpiderMonkey
* JavaScriptCore
* JerryScript
* Duktape
* ChakraCore
* MuJS

## Decision Outcome

Chosen option: ["QuickJS" (quickjs-wasm-rs)](https://crates.io/crates/quickjs-wasm-rs) in combination with [Wasmtime](https://wasmtime.dev/), because it is the best option for our use case.

## Pros and Cons of the Options

### V8 (Rusty V8 Binding)

- Performance

    - No data founded

- Memory usage
    
    - Good, because V8 has the capability to restrict it.

        TODO: add a reference

- Size of the binary

    - Bad, because Rust bindings to V8 size is 19.8 MB.

- Compatibility with the WASI API

    - No data founded

- Compatibility with the Rust ecosystem:

    - Good, because there is a crate, [Rusty V8 Binding](https://crates.io/crates/v8), maintained by the [Deno](https://github.com/denoland/rusty_v8) team.

- Ease of use

    - No data founded

- Documentation

    - Good, at least they have a [documentation](https://docs.rs/v8).

- Community support:

    - Good, because it is the JS engine used by [Node.js](https://nodejs.org/en/).
    - Good, because the Rusty V8 binding is maintained by the [Deno](https://deno.land/) team.

- License:

    - Good, because it is licensed under the [MIT license](https://github.com/denoland/rusty_v8/blob/main/LICENSE).

- Security

    - Okay, because it use isolate to run untrusted code.
    
        TODO: add a reference about how trustable it is

- Stability

    - No data founded

### QuickJS (quickjs-wasm-rs)

- Performance

    - Good, because it is Small and easily embeddable.
    - Good, because it is fast interpreter with very low startup time.

    Ref: [QuickJS](https://bellard.org/quickjs/)

- Memory usage

    - Good, because it has the capability to restrict it using [Wasmtime](https://wasmtime.dev/).
    
        Ref.:
        - https://github.com/bytecodealliance/wasmtime/issues/2273
        - https://github.com/bytecodealliance/wasmtime/issues/2274

- Size of the binary

    - Good, because the size of wasm file created from wasm-workers-server is 3.0 MB.

- Compatibility with the WASI API

    - Good, it is created to be build to WASI. 

- Compatibility with the Rust ecosystem

    - Good, because it is a Rust crate, [quickjs-wasm-rs](https://crates.io/crates/quickjs-wasm-rs), maintained by the [Shopify](https://github.com/Shopify/javy/tree/main/crates/quickjs-wasm-rs) team.

- Ease of use

    - No data founded

- Documentation

    - Bad, lack of documentation. Some projects are using it, so their code can help.
    - Okay, because there is the QuickJS documentation.

- Community support

    - Bad, there isn't a community behind.
    - Good, some projects are using it. There is the possibility to have a community in the future.

- License

    - Good, because it is licensed under the [Apache-2.0](https://github.com/Shopify/javy/blob/main/LICENSE.md) license

- Security

    - Good, because by default, the WASI process does not have access to the file system.

- Stability

    - No data founded
