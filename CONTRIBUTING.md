# Contributing

## Core facets:

### [Rust] The Pagefind indexing binary
This lives in the `pagefind` directory, and houses the code for indexing a built static site.

### [JavaScript] The Pagefind search interface
This lives in the `pagefind_web_js` directory.

### [Rust] The Pagefind WebAssembly
This lives in `pagefind_web`, and is what performs the actual search actions in the browser.

### [JavaScript] The Pagefind UI modules
These are the node packages in `pagefind_ui`, which are both published to NPM and compiled into the indexing binary.

### [JavaScript] The wrapper Node module
This lives in `wrappers/node`, and is what provides the `npx pagefind` binary runner, as well as the Node bindings for Pagefind.

## Extras:

### [Hugo] The Pagefind documentation
This lives in `docs`, and is the static site generating the content at https://pagefind.app

### [Rust] The Pagefind stemmer
This lives in `pagefind_stem`, and it's unlikely you'll need to touch this.

***

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [NodeJS](https://nodejs.org/en/download)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Add the wasm target to your Rust installation: `rustup target add wasm32-unknown-unknown`

NB: Contributing right now is certainly easier on macOS/Linux systems, but there are no blockers for contributing from Windows.
To do so, you'll just need to run the applicable commands from the bash scripts in the repo manually (adjusted for Windows as needed).

## Building the supporting packages

Ultimately, most contributions will require the ability to build the main Pagefind binary.
That binary compiles in some of our supporting facets, so you'll need to build those first.

First, build the web JS bindings with:
- `cd pagefind_web_js && npm i && npm run build-coupled`

Next, build the UI packages with:
- `cd pagefind_ui/default && npm i && npm run build`
- `cd pagefind_ui/modular && npm i && npm run build`

This builds the packages for distribution, but also builds the files to the `pagefind/vendor` directory,
which is where the Pagefind source looks for them during compilation.

Next, build the Playground:

- `cd pagefind_playground && npm i && npm run build`

Next, build the WebAssembly package with:
- `cd pagefind_web && ./local_build.sh`

Similar to before, this builds the WebAssembly outputs to the `pagefind/vendor/wasm` directory.
This step might take a while, as it needs to build a WASM file for each supported language. Grab a tea 🙂

## Building the main package

To build the main Pagefind binary, enter the `pagefind` folder and run `cargo build --release --features extended`.
Pagefind currently runs _very_ slowly in a debug build, so the extra time of a `--release` compile is more than made up
for by the faster runtime of the output binary, especially when running the test suite.

After building, you'll have a final Pagefind binary at `target/release/pagefind` (in the root of the repo, as we are a cargo workspace).

## Test suite

To run the integration test suite, from the root folder run `npx toolproof@latest`.
This will give you a terminal interface to run tests and accept snapshot changes.

From the `pagefind` directory you can run `cargo test` for unit tests.

For most changes unit tests are a nice to have, but integration tests are better.

You can see the integration test files inside `pagefind/integration_tests`. These are written for, and run by, Toolproof.
You can see documentation for this at https://toolproof.app/

## Manually testing

For the UI packages, running `npm start` in either the `pagefind_ui/default` or `pagefind_ui/modular` directories will
start serving a dev server with these UI libraries rendered on the page. Reload to automatically pull in any changes to files.

Currently this does just stub out a Pagefind mock, so for anything more substantial you'll want to run `npm run build`, then build
the main Pagefind package and test from there.

To test the main package, run the `target/release/pagefind` file however you would normally run Pagefind, and use the assets it creates
to test any dependent package.

A quick way to get off the ground is to test using the `docs` site in this repo. Enter the `docs` directory and follow the given steps:

- Delete any `public` directory if it already exists
- Run `npm i`
- Run `hugo` to build the site
- Run `../target/release/pagefind -s public --serve`
- Open the provided URL, and you should now see the Pagefind documentation, but:
  - Indexed by your local build of the binary
  - Searching with your local build of the WebAssembly
  - Shown with your local build of the Default UI

## Further Notes

TODOS:
- Devise and document a nice way to manually test the npx wrapper behaviour
- Devise and document a nice way to manually test the Node package interface
