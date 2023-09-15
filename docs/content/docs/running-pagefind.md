---
title: "Running Pagefind"
nav_title: "Running Pagefind"
nav_section: Indexing
weight: 1
---

Pagefind usually runs after your static site generator, ingesting your static HTML files and creating a static search index.

## Running the Pagefind CLI

Most use-cases are covered by using the Pagefind CLI, either directly or through Pagefind's npx wrapper library.

For help installing the Pagefind CLI, see [Installing Pagefind](/docs/installation/). These docs assume you're running via npx — if that isn't the case, just replace `npx pagefind` with the path to your binary.

The minimal command for Pagefind to index a site is:

```bash
npx pagefind --site public
```

The `--site` flag here should point to a directory of static HTML files. For example, the static site generator Hugo builds files into a `public` directory by default, so that should be Pagefind's `site` directory.

After running this command, you will see that Pagefind has added a directory at `public/pagefind`. This is your search bundle, and contains Pagefind's browser dependencies as well as the index files required to search your site.

Next steps:
- Most of Pagefind's indexing configuration happens inline in your HTML, [Configuring your index](/docs/indexing/) covers the important tags.
- If you prefer using config files or environment variables over CLI flags, this is possible too. See the [configuration sources](/docs/config-sources/) reference for more.
- To see all available options when running the CLI, see the [configuration options](/docs/config-options/) reference.

## Running the NodeJS indexing API

Pagefind also exposes a NodeJS interface that can be used to programmatically build an index. Using this, you can index non-static websites, or even non-HTML content altogether. The NodeJS library can also be used to integrate Pagefind into developer tooling for static websites.

You can find all of the details for this library on the [Indexing content using the NodeJS API](/docs/node-api/) page.