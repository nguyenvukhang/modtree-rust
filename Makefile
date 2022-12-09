test:
	cargo test --workspace

build:
	cargo build --workspace --release
	./target/release/fetcher

.PHONY: test build
