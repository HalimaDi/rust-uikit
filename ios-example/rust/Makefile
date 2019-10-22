
CARGO_MODE ?= debug
LIB = libuikit_example.a

all:
	cargo lipo
	cp target/universal/debug/$(LIB) ./

doc:
	cargo doc --target aarch64-apple-ios --open
