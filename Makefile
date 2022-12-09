test:
	cargo test --workspace

build:
	cargo build --release
	./target/release/modtree

.PHONY: test build
