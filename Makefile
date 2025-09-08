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
