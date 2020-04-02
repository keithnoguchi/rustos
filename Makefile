# SPDX-License-Identifier: GPL-2.0
TARGETS	:= post01
TARGETS	+= post02

.PHONY: init fmt lint image run
all: fmt lint $(TARGETS) main image
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
fmt:
	@rustfmt --edition 2018 --check **/*.rs
lint:
	@cargo clippy -- -D warnings
image:
	@cargo bootimage --target x86_64-os.json
run:
	@cargo xrun --target x86_64-os.json
run-%:
	@cargo xrun --target x86_64-os.json --example $*
