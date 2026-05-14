# AuroraOS Makefile

.PHONY: all build clean run test docs check fmt clippy

# Configuration
ARCH ?= x86_64
TARGET := $(ARCH)-unknown-none-aurora
BUILD_MODE := release
KERNEL_BIN := target/$(TARGET)/$(BUILD_MODE)/aurora_kernel
BOOTLOADER_BIN := target/x86_64-unknown-uefi/$(BUILD_MODE)/bootloader.efi
DISK_IMAGE := auroraos.img
ISO_IMAGE := auroraos.iso

# Rust tools
RUSTC := rustup run nightly rustc
CARGO := rustup run nightly cargo
OBJCOPY := llvm-objcopy
OBJDUMP := llvm-objdump

all: build

# Build everything
build: kernel bootloader userspace

# Build kernel
kernel:
	@echo "Building kernel..."
	$(CARGO) build --package aurora_kernel --target config/$(TARGET).json --profile $(BUILD_MODE)
	@echo "Creating kernel binary..."
	$(OBJCOPY) -O binary $(KERNEL_BIN) kernel.bin

# Build bootloader
bootloader:
	@echo "Building UEFI bootloader..."
	$(CARGO) build --package aurora_bootloader --target x86_64-unknown-uefi --profile $(BUILD_MODE)
	@cp $(BOOTLOADER_BIN) bootloader.efi 2>/dev/null || true

# Build userspace components
userspace:
	@echo "Building userspace..."
	$(CARGO) build --package aurora_userspace --target $(ARCH)-unknown-aurora-gnu --profile $(BUILD_MODE)
	$(CARGO) build --package aurora_shell --target $(ARCH)-unknown-aurora-gnu --profile $(BUILD_MODE)
	$(CARGO) build --package aurora_apps --target $(ARCH)-unknown-aurora-gnu --profile $(BUILD_MODE)

# Create disk image
disk-image: build
	@echo "Creating disk image..."
	./scripts/tools/disk-image.sh

# Create ISO
iso: build
	@echo "Creating ISO image..."
	./scripts/tools/iso-builder.sh

# Run in QEMU
run: build
	@echo "Running AuroraOS in QEMU..."
	qemu-system-x86_64 \
		-cpu qemu64 \
		-m 2G \
		-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
		-drive if=pflash,format=raw,file=OVMF_VARS.fd \
		-drive format=raw,file=$(DISK_IMAGE) \
		-serial stdio \
		-vga virtio \
		-device virtio-keyboard-pci \
		-device virtio-mouse-pci \
		-netdev user,id=net0 \
		-device e1000,netdev=net0

# Run with GDB
run-gdb: build
	@echo "Running AuroraOS in QEMU with GDB..."
	qemu-system-x86_64 \
		-cpu qemu64 \
		-m 2G \
		-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
		-drive if=pflash,format=raw,file=OVMF_VARS.fd \
		-drive format=raw,file=$(DISK_IMAGE) \
		-serial stdio \
		-vga virtio \
		-s -S

# Check code
check:
	$(CARGO) check --workspace --all-targets

# Format code
fmt:
	$(CARGO) fmt --all

# Run clippy
clippy:
	$(CARGO) clippy --workspace --all-targets -- -D warnings

# Run tests
test:
	@echo "Running tests..."
	$(CARGO) test --workspace

# Generate documentation
docs:
	@echo "Generating documentation..."
	$(CARGO) doc --workspace --no-deps

# Clean build artifacts
clean:
	@echo "Cleaning..."
	$(CARGO) clean
	rm -f kernel.bin bootloader.efi $(DISK_IMAGE) $(ISO_IMAGE)

# Disassemble kernel
disasm:
	$(OBJDUMP) -d $(KERNEL_BIN) > kernel.disasm
	@echo "Disassembly written to kernel.disasm"

# Help
help:
	@echo "AuroraOS Build System"
	@echo ""
	@echo "Targets:"
	@echo "  all          - Build everything (default)"
	@echo "  build        - Build kernel, bootloader, and userspace"
	@echo "  kernel       - Build kernel only"
	@echo "  bootloader   - Build UEFI bootloader"
	@echo "  userspace    - Build userspace components"
	@echo "  disk-image   - Create disk image"
	@echo "  iso          - Create ISO image"
	@echo "  run          - Run in QEMU"
	@echo "  run-gdb      - Run in QEMU with GDB server"
	@echo "  check        - Check code without building"
	@echo "  fmt          - Format all code"
	@echo "  clippy       - Run clippy lints"
	@echo "  test         - Run tests"
	@echo "  docs         - Generate documentation"
	@echo "  clean        - Remove build artifacts"
	@echo "  disasm       - Disassemble kernel"
	@echo ""
	@echo "Variables:"
	@echo "  ARCH         - Target architecture (default: x86_64)"
	@echo "  BUILD_MODE   - Build profile (default: release)"
