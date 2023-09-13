---
title: "Troubleshoot Hosting Pagefind"
nav_title: "Troubleshoot Hosting"
nav_section: Resources
weight: 81
---

Pagefind outputs a static bundle directory to your built site, and no hosting configuration is required.

## Compression

Pagefind handles compression of the files in the bundle directly, so no server gzip support is required.

## Content Security Policy (CSP)

If you have a strict content security policy enabled on your site, you may encounter issues with the Pagefind WebAssembly — this isn't specific to Pagefind but is an issue with CSP and WebAssembly.

The most widely-supported solution at the current moment is to ensure your Content Security Policy allows `script-src 'unsafe-eval'`, which will work in all browsers.

A [proposal exists](https://github.com/WebAssembly/content-security-policy/blob/main/proposals/CSP.md) for `script-src 'wasm-unsafe-eval'`, which is supported in Chrome, Firefox, and Edge, but has not yet shipped to a stable Safari version.

> In the future, hopefully a `wasm-src` attribute / SRI hash validation will be supported in CSP, as proposed in [chrome#961485](https://bugs.chromium.org/p/chromium/issues/detail?id=961485), [chrome#945121](https://bugs.chromium.org/p/chromium/issues/detail?id=945121).  
[Open an issue](https://github.com/CloudCannon/pagefind/issues) if this is now the case!

If you're using the Pagefind UI snippet as documented you will also need `unsafe-inline`, but this could also be addressed by moving the Pagefind initialization into one of your existing JavaScript files.
