all: build strip

setup:
	rustup override set nightly

build:
	cargo build --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-pc-windows-msvc

strip:
	strip target/x86_64-pc-windows-msvc/release/minerva.exe

.PHONY: all build strip