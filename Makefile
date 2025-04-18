.PHONY: build-win build-mac

build-win:
	cargo build --target x86_64-pc-windows-gnu --release

build-mac:
	cargo build --target aarch64-apple-darwin --release