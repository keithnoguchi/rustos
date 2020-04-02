# SPDX-License-Identifier: GPL-2.0
TARGETS	:= post01 # A Freestanding Rust Binary
TARGETS	+= post02 # A Minimal Rust Kernel
TARGETS	+= post03 # VGA Text Mode

.PHONY: init update fmt lint test image run
all: fmt lint $(TARGETS) image
main:
	@cargo xbuild --target x86_64-os.json
$(TARGETS):
	@cargo xbuild --target x86_64-os.json --example $@
init:
	@rustup update nightly
	@rustup default nightly
	@cargo install cargo-xbuild
	@cargo install bootimage
	@rustup component add rust-src
	@rustup component add llvm-tools-preview
update: init
	@cargo update
fmt:
	@rustfmt --edition 2018 --check **/*.rs
lint:
	@cargo clippy -- -D warnings
image: main
	@cargo bootimage --target x86_64-os.json
run: image
	@cargo xrun --target x86_64-os.json
run-%:
	@cargo xrun --target x86_64-os.json --example $*
