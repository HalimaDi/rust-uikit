
CARGO_MODE ?= debug
LIB = libuikit_example.a

all:
	#cargo lipo
	cargo build --target x86_64-apple-ios
	cp target/x86_64-apple-ios/debug/$(LIB) ./

doc:
	cargo doc --target aarch64-apple-ios --open
