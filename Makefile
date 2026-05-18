clean:
	@cargo clean
	@rm -Rf target

test:
	@RUST_BACKTRACE=1 cargo test -- --color=always --test-threads=1

lint:
	@cargo clippy --all-targets

update:
	@cargo update