# Makefile for cross-compiling Horizon

# Rust compiler
RUSTC := rustc
CARGO := cargo

# Default target
DEFAULT_TARGET := x86_64-unknown-linux-gnu

# List of targets to build for
TARGETS := \
	x86_64-unknown-linux-gnu \
	i686-unknown-linux-gnu \
	aarch64-unknown-linux-gnu \
	armv7-unknown-linux-gnueabihf \
	x86_64-pc-windows-gnu \
	x86_64-apple-darwin

# List of common Linux distributions
DISTROS := \
	ubuntu \
	debian \
	fedora \
	centos \
	arch

# Output directory
OUT_DIR := target/platforms

# Default release mode
RELEASE_MODE := release

.PHONY: all clean $(TARGETS) $(DISTROS)

all: $(TARGETS)

# Rule for building each target
$(TARGETS):
	@echo "Building for $@"
	@mkdir -p $(OUT_DIR)/Release-$@
	@rustup target add $@
	@RUSTFLAGS="-C target-feature=+crt-static" $(CARGO) build --target $@ --$(RELEASE_MODE)
	@cp target/$@/$(RELEASE_MODE)/horizon $(OUT_DIR)/Release-$@/

# Rule for building for each Linux distribution
$(DISTROS):
	@echo "Building for $@ (x86_64)"
	@mkdir -p $(OUT_DIR)/Release-$@-x86_64-unknown-linux-gnu
	@$(CARGO) build --target x86_64-unknown-linux-gnu --$(RELEASE_MODE)
	@cp target/x86_64-unknown-linux-gnu/$(RELEASE_MODE)/horizon $(OUT_DIR)/Release-$@-x86_64-unknown-linux-gnu/

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts"
	@rm -rf $(OUT_DIR)
	@$(CARGO) clean

# Help target
help:
	@echo "Available targets:"
	@echo "  all        - Build for all targets"
	@echo "  clean      - Clean build artifacts"
	@echo "  $(TARGETS)"
	@echo "  $(DISTROS)"
	@echo ""
	@echo "Usage:"
	@echo "  make [target]"
	@echo "  make RELEASE_MODE=debug [target]  # Build in debug mode"

# Default target
.DEFAULT_GOAL := help