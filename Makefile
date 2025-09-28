SHELL := /bin/bash

BIN := runbeam
UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)
TMP := ./tmp
VERSION := $(shell sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)

.DEFAULT_GOAL := help

.PHONY: help
help:
	@echo "Common targets:"
	@echo "  make build            # Debug build"
	@echo "  make release          # Release build (optimized)"
	@echo "  make package-macos    # Package macOS binary (current arch) into ./tmp"
	@echo "  make package-linux    # Package Linux (x86_64 musl) into ./tmp (requires musl toolchain)"
	@echo "  make package-windows  # Package Windows (x86_64 msvc) into ./tmp (builds only on Windows)"
	@echo "  make clean-artifacts  # Remove ./tmp artifacts"

$(TMP):
	@mkdir -p $(TMP)

.PHONY: build
build:
	cargo build

.PHONY: release
release:
	cargo build --release

# macOS packaging (current host arch)
.PHONY: package-macos
package-macos: release $(TMP)
	@OUT=$(TMP)/$(BIN)-macos-$(UNAME_M)-v$(VERSION); \
	cp target/release/$(BIN) $$OUT; \
	if [ "$(UNAME_S)" = "Darwin" ]; then strip -x $$OUT || true; fi; \
	tar -C $(TMP) -czf $$OUT.tar.gz $$(basename $$OUT); \
	shasum -a 256 $$OUT.tar.gz > $$OUT.tar.gz.sha256; \
	ls -lh $$OUT*

# Linux static build/package (x86_64 musl). Requires: rustup target add x86_64-unknown-linux-musl and musl tools (or cargo-zigbuild).
.PHONY: package-linux
package-linux: $(TMP)
	rustup target add x86_64-unknown-linux-musl || true
	@if command -v cargo zigbuild >/dev/null 2>&1; then \
		cargo zigbuild --release --target x86_64-unknown-linux-musl; \
	else \
		cargo build --release --target x86_64-unknown-linux-musl; \
	fi
	@OUT=$(TMP)/$(BIN)-linux-x86_64-v$(VERSION); \
	cp target/x86_64-unknown-linux-musl/release/$(BIN) $$OUT; \
	strip --strip-unneeded $$OUT || true; \
	tar -C $(TMP) -czf $$OUT.tar.gz $$(basename $$OUT); \
	shasum -a 256 $$OUT.tar.gz > $$OUT.tar.gz.sha256; \
	ls -lh $$OUT*

# Windows packaging (x86_64 msvc). Typically run on Windows.
.PHONY: package-windows
package-windows: $(TMP)
	rustup target add x86_64-pc-windows-msvc || true
	cargo build --release --target x86_64-pc-windows-msvc
	@OUT=$(TMP)/$(BIN)-windows-x86_64-v$(VERSION).exe; \
	cp target/x86_64-pc-windows-msvc/release/$(BIN).exe $$OUT; \
	cd $(TMP) && zip -9 $$(basename $$OUT .exe).zip $$(basename $$OUT); \
	cd $(TMP) && certutil -hashfile $$(basename $$OUT .exe).zip SHA256 | sed -n '2p' > $$(basename $$OUT .exe).zip.sha256 || shasum -a 256 $$(basename $$OUT .exe).zip > $$(basename $$OUT .exe).zip.sha256; \
	ls -lh $(TMP)/*windows*

.PHONY: clean-artifacts
clean-artifacts:
	rm -rf $(TMP)
