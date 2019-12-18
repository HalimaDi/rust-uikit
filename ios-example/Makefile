.PHONY: xcode cargo

run: install
	RUST_LOG=debug RUST_BACKTRACE=full xcrun simctl launch booted com.sasheldon.RustUIKitExample

install: xcode
	xcrun simctl install booted build/Build/Products/Debug-iphonesimulator/RustUIKitExample.app

xcode: cargo
	xcrun xcodebuild -scheme RustUIKitExample -project RustUIKitExample.xcodeproj/ -configuration Debug -destination 'platform=iOS Simulator,name=iPhone 8,OS=13.1' -derivedDataPath build

cargo:
	make -C rust

simulator-logs:
	RUST_LOG=debug RUST_BACKTRACE=full xcrun simctl spawn booted log stream --level=debug --predicate 'processImagePath endswith "RustUIKitExample"'
