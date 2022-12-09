test:
	cargo test --workspace

build:
	@make build-debug

build-debug:
	cargo build --workspace
	./target/debug/database

build-release:
	cargo build --workspace --release
	./target/release/database

.PHONY: test build build-debug build-release
