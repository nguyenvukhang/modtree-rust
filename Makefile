# @sh scripts/debug.sh release database
# @sh scripts/debug.sh release fetcher

test:
	cargo test --workspace

build:
	@sh scripts/debug.sh debug database

.PHONY: test build
