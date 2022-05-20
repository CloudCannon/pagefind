## Tests

- `TEST_BINARY=../target/release/pagefind cargo test --release --test cucumber -- -c 16 --tags "not @skip"`
- `TEST_BINARY=../target/release/pagefind cargo test --release --test cucumber -- --name "<test>"`