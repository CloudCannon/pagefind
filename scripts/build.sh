#!/usr/bin/env bash
set -eou pipefail

# export GIT_VERSION=$(git describe --tags --always --dirty)
export GIT_VERSION="1.3.1"

cd pagefind_web_js && npm i && npm run build-coupled && cd ..
echo "✅ 1/6 pagefind_web_js built successfully"
# cd pagefind_web && ./local_build.sh && cd ..
cd pagefind_ui/default && npm i && npm run build && cd ../..
echo "✅ 2/6 pagefind_ui/default built successfully"
cd pagefind_ui/modular && npm i && npm run build && cd ../..
echo "✅ 3/6 pagefind_ui/modular built successfully"
cd pagefind_playground && npm i && npm run build && cd ..
echo "✅ 4/6 pagefind_playground built successfully"
# cd pagefind_web && ./local_build.sh && cd ..
cd pagefind_web && ./local_fast_build.sh && cd ..
# Check previous exit code
if [ $? -ne 0 ]; then
  echo "Build failed"
  exit 1
else
  echo "✅ 5/6 pagefind_web built successfully"
fi

cd pagefind && cargo build --release --features extended && cd ..
echo "✅ 6/6 pagefind built successfully"
echo "All components built successfully"
echo "To run the playground, use: "

