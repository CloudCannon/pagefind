#!/usr/bin/env bash
export files_to_cog=(
  README.md 
  src/pagefind_python/__init__.py
  pyproject.toml
)
# you can check this list by running `rg -l '\[\[\[cog' ./` in the repo root
