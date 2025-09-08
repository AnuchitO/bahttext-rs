.PHONY: test
test:
	cargo test

.PHONY: test-watch
test-watch:
	cargo watch test

.PHONY: test-watch-words
test-watch-words:
	cargo watch -x "test words"

.PHONY: test-cov
test-cov:
	cargo tarpaulin --out html
	open tarpaulin-report.html

.PHONY: install-dev-tools
install-dev-tools:
	cargo install cargo-watch
	cargo install cargo-tarpaulin
	cargo install cargo-fuzz

.PHONY: fuzz-init
fuzz-init:
	cargo fuzz init

.PHONY: fuzz
fuzz:
	cargo fuzz run fuzz_target_1 -- -max_total_time=60
