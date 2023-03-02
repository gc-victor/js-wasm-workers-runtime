# JS WASM Workers Runtime

The JS WASM Workers Runtime is a WebAssembly runtime that uses QuickJS as the JavaScript engine. It is designed to be used in a Rust environment.

This project is based on the [quickjs-wasm-rs](https://github.com/Shopify/javy/tree/main/crates/quickjs-wasm-rs) crate from Shopify. And gets inspiration from other JS runtime libraries like: 

- [spin-js-sdk](https://github.com/fermyon/spin-js-sdk)
- [wasm-workers-server](https://github.com/vmware-labs/wasm-workers-server)

## Build

To build the project, you need to have Rust installed. You can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

NodeJS and pnpm are also required. You can install them from [https://nodejs.org/en/download/](https://nodejs.org/en/download/) and [https://pnpm.io/installation](https://pnpm.io/installation).

Once you have installed all the dependencies, you can build the project by running:

```bash
make build
```

## Architecture Decisions

We use [ADR](https://adr.github.io/) to document architecture decisions. You can find them in the [docs/decisions](/docs/decisions) folder.

## Compatible Versioning

### Summary

Given a version number MAJOR.MINOR, increment the:

- MAJOR version when you make backwards-incompatible updates of any kind
- MINOR version when you make 100% backwards-compatible updates

Additional labels for pre-release and build metadata are available as extensions to the MAJOR.MINOR format.

More information about [Compatible Versioning](https://gitlab.com/staltz/comver).

## Contribute

First off, thanks for taking the time to contribute!
Now, take a moment to be sure your contributions make sense to everyone else.

### Reporting Issues

Found a problem? Want a new feature? First of all, see if your issue or idea has [already been reported](../../issues).
If it hasn't, just open a [new clear and descriptive issue](../../issues/new).

### Commit message conventions

A specification for adding human and machine readable meaning to commit messages.

- [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)

### Submitting pull requests

Pull requests are the greatest contributions, so be sure they are focused in scope and do avoid unrelated commits.

-   Fork it!
-   Clone your fork
-   Navigate to the newly cloned directory
-   Create a new branch for the new feature
-   Install the tools necessary for development
-   Make your changes.
-   Verify your change doesn't increase output size.
-   Make sure your change doesn't break anything.
-   Commit your changes
-   Push to the branch
-   Submit a pull request with full remarks documenting your changes.