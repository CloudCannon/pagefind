#!/usr/bin/env bash
export TWINE_REPOSITORY=testpypi
export TWINE_USERNAME=__token__
export TWINE_PASSWORD="${TEST_PYPI_API_TOKEN:?missing TEST_PYPI_API_TOKEN}"
python3 -m twine upload --verbose ./dist/*
