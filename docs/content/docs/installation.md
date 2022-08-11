---
date: 2022-06-01
title: "Installing and running Pagefind"
nav_title: "Installation"
nav_section: Installing
weight: 1
---

Pagefind is a static binary with no dynamic dependencies, so in most cases will be simple to install and run. Pagefind is currently supported on Windows, MacOS, and x86-64 Linux distributions.

## Running via npx

```bash
npx pagefind --source "public"
```

Pagefind publishes a [wrapper package through npm](https://www.npmjs.com/package/pagefind), which is the easiest way to get started. This package will download the correct [binary of the latest release](https://github.com/CloudCannon/pagefind/releases) from GitHub for your platform and run it.

Specific versions can be run by passing a version tag:

```bash
npx pagefind@latest --source "public"

npx pagefind@v0.2.0 --source "public"
```

> Running Pagefind via npx will download the `pagefind_extended` release, which includes specialized support for indexing Chinese and Japanese pages.

## Downloading a precompiled binary

If you prefer to install Pagefind yourself, you can download a [precompiled release from GitHub](https://github.com/CloudCannon/pagefind/releases) and run the binary directly:

```bash
./pagefind --source "public"
# or
./pagefind_extended --source "public"
```

Pagefind publishes two releases, `pagefind` and `pagefind_extended`. The extended release is a larger binary, but includes specialized support for indexing Chinese and Japanese pages.

## Building from source

If you have [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed, you can run `cargo install pagefind` to build from source.

```bash
cargo install pagefind
pagefind --source "public"
```

To build and install the extended version of Pagefind:

```bash
cargo install pagefind --features extended
pagefind --source "public"
```
