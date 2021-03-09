SRC := $(wildcard src/*.rs)
TARGET_WIN :=x86_64-pc-windows-gnu
TARGET_LINUX := x86_64-unknown-linux-gnu
BIN_NAME := tagsearch

.PHONY: build build_linux64 build_win64 check clean

$(BIN_NAME): build

build: build_win64 build_linux64

build_linux64: $(SRC)
	cargo build --release --target=$(TARGET_LINUX)

build_win64: $(SRC)
	cargo build --release --target=$(TARGET_WIN)

debug_linux64: $(SRC)
	cargo build --target=$(TARGET_LINUX)

debug_win64: $(SRC)
	cargo build --target=$(TARGET_WIN)

check:
	cargo watch -x check

clean:
	cargo clean

release: $(BIN_NAME)
	$$(rg version Cargo.toml | head -n1 | sed -e 's/.*"\([0-9]\+.[0-9]\+.[0-9]\+\)".*/\1/g' > VERSION)
	gh release create "v$$(cat VERSION)" --title "Release $$(cat VERSION)" target/$(TARGET_WIN)/release/$(BIN_NAME).exe target/$(TARGET_LINUX)/release/$(BIN_NAME)
	git fetch --tags origin
	rm VERSION
