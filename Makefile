
.PHONY: build check test doc

build: check
	cargo doc
	cargo build

check:
	cargo clippy
	cargo fmt --check

test: check
	RUSTDOCFLAGS="--cfg test" cargo test --doc -v -- --show-output

doc: build
	open target/doc/mockdown/index.html
