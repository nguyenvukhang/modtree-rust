# @sh scripts/debug.sh release database
# @sh scripts/debug.sh release fetcher

build:
	# @sh scripts/debug.sh debug database
	@cargo build --workspace
	@./target/debug/modtree

test:
	cargo test --workspace

.PHONY: test build
