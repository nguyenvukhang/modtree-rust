# @sh scripts/debug.sh release database
# @sh scripts/debug.sh release fetcher

call:
	make test

test:
	cargo test --workspace

build:
	# @sh scripts/debug.sh debug database
	@cargo build --workspace
	@./target/debug/modtree

build_release:
	# @sh scripts/debug.sh debug database
	@cargo build --workspace --release
	@./target/release/modtree

.PHONY: test build call build_release
