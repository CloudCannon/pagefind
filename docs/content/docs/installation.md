---
title: "Installing and running Pagefind"
nav_title: "Installing the CLI"
nav_section: References
weight: 49
---

Pagefind is a static binary with no dynamic dependencies, so in most cases will be simple to install and run. Pagefind is currently supported on Windows, macOS, and Linux distributions.

## Running via npx

For users with a NodeJS toolchain already installed, Pagefind publishes a [wrapper package through npm](https://www.npmjs.com/package/pagefind):

```bash
npx pagefind --site "public"
```

This package includes the correct [binary of the relevant release](https://github.com/CloudCannon/pagefind/releases) as a dependency for your platform.

Specific versions can be run by passing a version tag:

```bash
npx pagefind@latest --site "public"

npx pagefind@v1.1.1 --site "public"
```

Running Pagefind via npx will always download the `pagefind_extended` release, which includes specialized support for indexing Chinese and Japanese pages.

> Pagefind's npm package can also be imported and controlled from a script. See the [Node API documentation](/docs/node-api/) for details.

## Running via Python

For users with a Python toolchain already installed, Pagefind publishes a [wrapper package through pypi](https://pypi.org/project/pagefind/):

```bash
python3 -m pip install 'pagefind[extended]'
python3 -m pagefind --site "public"
```

This package includes the correct [binary of the relevant release](https://github.com/CloudCannon/pagefind/releases) as a dependency for your platform.

Specific versions can be installed by passing a version:

```bash
python3 -m pip install 'pagefind[extended]==1.1.1'
```

The above example shows installing the `pagefind_extended` release, which includes specialized support for indexing Chinese and Japanese pages.
To install the smaller standard release, run:

```bash
python3 -m pip install 'pagefind[bin]'
```

> Pagefind's Python package can also be imported and controlled from a script. See the [Python API documentation](/docs/py-api/) for details.

## Downloading a precompiled binary

If you prefer to install Pagefind yourself, you can download a [precompiled release from GitHub](https://github.com/CloudCannon/pagefind/releases) and run the binary directly:

```bash
./pagefind --site "public"
# or
./pagefind_extended --site "public"
```

Pagefind publishes two releases, `pagefind` and `pagefind_extended`. The extended release is a larger binary, but includes specialized support for indexing Chinese and Japanese pages.

## Building from source

If you have [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed, you can run `cargo install pagefind` to build from source.

```bash
cargo install pagefind
pagefind --site "public"
```

To build and install the extended version of Pagefind:

```bash
cargo install pagefind --features extended
pagefind --site "public"
```
