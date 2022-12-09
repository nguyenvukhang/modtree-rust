test:
	cargo test --workspace

build:
	@sh scripts/debug.sh debug database
	# @sh scripts/debug.sh release database

.PHONY: test build
