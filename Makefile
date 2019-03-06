.PHONY: all
all: loader

.PHONY: kernel
kernel:
	cd kernel && cargo xbuild --target ./higherhalf.json

.PHONY: loader
loader: kernel
	cd loader && cargo xbuild --target x86_64-unknown-uefi

.PHONY: run
run: all
	uefi-run ./target/x86_64-unknown-uefi/debug/loader.efi
