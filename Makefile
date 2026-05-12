clean:
	@cargo clean
	rm -Rf target

test:
	@cargo test -- --no-capture --test-threads=1