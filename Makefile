test:
	cargo test --workspace

build:
	cargo build --workspace --release
	./target/release/database

.PHONY: test build
