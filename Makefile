ifdef DEBUG
  CARGOFLAGS =
  export KERNEL_TARGET_DIR = ./target/higherhalf/debug
  LOADER_TARGET_DIR = ./target/x86_64-unknown-uefi/debug
else
  CARGOFLAGS = --release
  export KERNEL_TARGET_DIR = ./target/higherhalf/release
  LOADER_TARGET_DIR = ./target/x86_64-unknown-uefi/release
endif

.PHONY: all
all: loader

.PHONY: kernel
kernel:
	cd kernel && cargo xbuild --target ./higherhalf.json $(CARGOFLAGS)

.PHONY: loader
loader: kernel
	cd loader && cargo xbuild --target x86_64-unknown-uefi $(CARGOFLAGS)

.PHONY: run
run: all
	uefi-run $(LOADER_TARGET_DIR)/loader.efi
