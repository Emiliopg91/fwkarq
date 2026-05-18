test:
	@RUST_BACKTRACE=1 cargo test -- --color=always --test-threads=1 --nocapture

lint:
	@cargo clippy --all-targets

clean:
	@cargo clean
	@rm -Rf target

update:
	@if [[ -n $$(git status --porcelain) ]]; then \
		echo "Cannot update dependencies version on dirty workspace"; \
	else \
		cargo update; \
	fi