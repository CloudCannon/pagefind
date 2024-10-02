Script to run tests from the repo root on an M* macOS:

```py
bin="$PWD/target/release/pagefind"
ext="$PWD/target/release/pagefind_extended"

cd wrappers/python

# set up the python virtual environment
poetry install --no-root # for dev dependencies
export VIRTUAL_ENV="${PWD}/.venv"
export PATH="$VIRTUAL_ENV/bin:$PATH"

# build and install the binary-only wheels

python3 -m scripts.build.binary_only_wheel \
  --llvm-triple="aarch64-apple-darwin" \
  --bin-path=$bin \
  --version=1.1.0

python3 -m scripts.build.binary_only_wheel \
  --llvm-triple="aarch64-apple-darwin" \
  --bin-path=$ext \
  --version=1.1.0

python3 -m scripts.build.api_package

poetry build # build the source-only distribution for the python API
# install all the wheels
pip install ./dist/*.whl --force-reinstall
pip show --verbose pagefind
pip show --verbose pagefind_bin
pip show --verbose pagefind_bin_extended
python3 --version

LOG_LEVEL="DEBUG" python3 ./src/tests/integration.py 2>&1 | tee /tmp/integration_test.log
```
