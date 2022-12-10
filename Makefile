test:
	cargo test --workspace

build:
	@sh scripts/debug.sh release fetcher
	# @sh scripts/debug.sh release fetcher
	# @sh scripts/debug.sh release database

.PHONY: test build
