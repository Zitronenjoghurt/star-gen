.PHONY: build win mac

build: win mac trace

trace:
	cargo run --release --features bevy/trace_tracy -- --codegen-units 16 --strip false --lto false --panic 'unwind'

win:
	@if [ -z "$(v)" ]; then echo "Error: Version parameter is required. Use 'make win v=x.y.z'"; exit 1; fi
	cargo build --target x86_64-pc-windows-gnu --release
	mkdir -p build/windows/v$(v)
	cp target/x86_64-pc-windows-gnu/release/star-gen.exe "build/windows/v$(v)/Star Generator v$(v) unsigned.exe"
	osslsigncode sign -certs signing/windows/codesign.crt -key signing/windows/codesign.key -n "Star Generator" -i "https://github.com/Zitronenjoghurt/star-gen" -in "build/windows/v$(v)/Star Generator v$(v) unsigned.exe" -out "build/windows/v$(v)/Star Generator v$(v).exe"
	cd build/windows/v$(v) && zip -r star-gen-v$(v)-win.zip "Star Generator v$(v).exe"
	@echo "Windows executable built and zipped"

mac:
	@if [ -z "$(v)" ]; then echo "Error: Version parameter is required. Use 'make mac v=x.y.z'"; exit 1; fi
	cargo bundle --target aarch64-apple-darwin --release
	mkdir -p build/macos/v$(v)
	cp -r "target/aarch64-apple-darwin/release/bundle/osx/Star Generator.app" "build/macos/v$(v)/Star Generator v$(v).app"
	codesign --force --deep --sign "https://github.com/Zitronenjoghurt" "build/macos/v$(v)/Star Generator v$(v).app"
	cd build/macos/v$(v) && zip -r star-gen-v$(v)-mac.zip "Star Generator v$(v).app"
	@echo "MacOS app bundle created and signed"