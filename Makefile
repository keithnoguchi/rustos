# SPDX-License-Identifier: Apache-2.0 OR MIT
TARGETS	:= post01 # A Freestanding Rust Binary
TARGETS	+= post02 # A Minimal Rust Kernel
TARGETS	+= post03 # VGA Text Mode
TARGETS	+= post04 # Testing
TARGETS	+= post05 # CPU Exceptions
TARGETS	+= post06 # Double Faults
TARGETS	+= post07 # Hardware Interrupts
TARGETS	+= post08 # Introduction Paging
TARGETS	+= post09 # Paing Implementation
TARGETS	+= post10 # Heap Allocation
TARGETS	+= post11 # Allocator Designs
TARGETS	+= post12 # Async/Await

CARGO	?= cargo
CARGO	+= -q
.PHONY: init update fmt lint doc image test run clean
all: fmt lint $(TARGETS) doc image test
main:
	@$(CARGO) xbuild --target x86_64-os.json
$(TARGETS):
	@$(CARGO) xbuild --target x86_64-os.json --example $@
init:
	@rustup update nightly
	@rustup default nightly
	@$(CARGO) install cargo-xbuild
	@$(CARGO) install bootimage
	@rustup component add rust-src
	@rustup component add llvm-tools-preview
update: init
	@$(CARGO) update
fmt:
	@rustfmt --edition 2018 --check **/*.rs
lint:
	@$(CARGO) clippy -- -D warnings
doc:
	@$(CARGO) doc
image:
	@$(CARGO) bootimage --target x86_64-os.json
test:
	@$(CARGO) xtest --target x86_64-os.json
test-%:
	@$(CARGO) xtest --target x86_64-os.json --test $*
run:
	@$(CARGO) xrun --target x86_64-os.json
run-%:
	@$(CARGO) xrun --target x86_64-os.json --example $*
clean:
	@$(CARGO) clean
