# SPDX-License-Identifier: GPL-2.0
TARGETS	:= post01
TARGETS	+= post02

.PHONY: fmt lint
all: fmt lint $(TARGETS) image
	@cargo xbuild --target x86_64-os.json
$(TARGETS):
	@cargo xbuild --target x86_64-os.json --example $@
fmt:
	@rustfmt --edition 2018 --check **/*.rs
lint:
	@cargo clippy -- -D warnings
image:
	@cargo bootimage --target x86_64-os.json
run:
	@cargo xrun --target x86_64-os.json
