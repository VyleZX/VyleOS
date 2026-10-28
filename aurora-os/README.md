# Aurora OS - A Modern Rust-Based Operating System

## Overview

Aurora OS is a next-generation operating system built primarily in Rust, designed from the ground up to be secure, performant, and developer-friendly. It features a hybrid microkernel architecture with userspace drivers, GPU-accelerated graphics, and a modern desktop environment.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                      Userspace Applications                      │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐   │
│  │ Browser │ │  IDE    │ │ Terminal│ │ Settings│ │  Mail   │   │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘   │
├─────────────────────────────────────────────────────────────────┤
│                    Userspace Services & Daemons                  │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌───────────┐ │
│  │ Compositor  │ │ Audio Server│ │ Network Mgr │ │ Power Mgr │ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └───────────┘ │
├─────────────────────────────────────────────────────────────────┤
│                     System Libraries (Rust)                      │
│  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌───────────────┐ │
│  │ libstd │ │ libgfx │ │ libnet │ │ libui  │ │ libaudio      │ │
│  └────────┘ └────────┘ └────────┘ └────────┘ └───────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│                         Kernel Space                             │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                    System Call Interface                   │  │
│  ├───────────┬───────────┬───────────┬───────────────────────┤  │
│  │  Process  │  Memory   │    IPC    │      Filesystem       │  │
│  │ Scheduler │  Manager  │  Subsystem│         VFS           │  │
│  ├───────────┴───────────┴───────────┴───────────────────────┤  │
│  │                    Device Drivers                          │  │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌─────────┐  │  │
│  │  │ Storage│ │Graphics│ │ Input  │ │Network │ │  USB    │  │  │
│  │  └────────┘ └────────┘ └────────┘ └────────┘ └─────────┘  │  │
│  └───────────────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│                        Hardware Layer                            │
│  CPU (x86_64/ARM64) │ RAM │ GPU │ NIC │ Storage │ Peripherals  │
└─────────────────────────────────────────────────────────────────┘
```

## Key Features

### Kernel Architecture
- **Hybrid Microkernel Design**: Minimal kernel with essential services, most drivers in userspace
- **Memory Management**: Advanced virtual memory, paging, ASLR, memory protection
- **Scheduler**: Preemptive multitasking with CFS-inspired algorithm, SMP support
- **IPC**: High-performance message passing, shared memory, pipes, sockets
- **Security**: Capability-based security model, sandboxing, secure boot

### Filesystem Support
- FAT32, exFAT, NTFS, EXT4, ISO9660, tmpfs
- Virtual Filesystem (VFS) layer for unified interface
- Journaling support, encryption, compression

### Hardware Support
- **Storage**: SATA, AHCI, NVMe, USB storage
- **Graphics**: VGA, Framebuffer, GPU acceleration, Vulkan-ready
- **Input**: USB HID, Keyboard, Mouse, Touchpad, Game controllers
- **Audio**: Intel HDA, Bluetooth audio
- **Networking**: Ethernet, Wi-Fi, TCP/IP, IPv4/IPv6, VPN
- **Power**: Battery management, Sleep/Hibernate, Thermal monitoring

### Graphics Stack
- GPU-accelerated compositor (Wayland-like protocol)
- Window manager with animations and effects
- Dark/Light themes, High DPI support
- Multi-monitor, Multi-workspace

### Security
- Memory protection, ASLR, DEP
- Sandboxing with capabilities
- Secure boot with signed drivers
- Encrypted storage, TPM support
- Firewall, SELinux-like mandatory access control

### Developer Experience
- First-class Rust APIs
- Comprehensive SDK and documentation
- Package manager with dependency resolution
- Debugging tools, profiler, emulator

## Directory Structure

```
aurora-os/
├── bootloader/          # UEFI/BIOS bootloader
├── kernel/              # Core kernel components
│   ├── src/
│   │   ├── arch/        # Architecture-specific code
│   │   ├── mm/          # Memory management
│   │   ├── sched/       # Process scheduler
│   │   ├── ipc/         # Inter-process communication
│   │   ├── syscall/     # System call handlers
│   │   ├── drivers/     # Kernel drivers
│   │   ├── fs/          # Filesystem implementations
│   │   ├── vfs/         # Virtual filesystem
│   │   ├── security/    # Security subsystem
│   │   └── panic/       # Panic handling
│   └── include/         # Kernel headers
├── userspace/           # Userspace components
│   ├── init/            # Init system and service manager
│   ├── compositor/      # GPU-accelerated compositor
│   ├── wm/              # Window manager
│   ├── gui-framework/   # Rust GUI toolkit
│   ├── apps/            # Default applications
│   ├── services/        # System services
│   └── libraries/       # System libraries
├── filesystems/         # Filesystem drivers
├── networking/          # Network stack
├── graphics/            # Graphics subsystem
├── security/            # Security components
├── drivers/             # Shared driver utilities
├── tools/               # Build and development tools
├── tests/               # Test suites
└── docs/                # Documentation
```

## Building

```bash
# Build the entire OS
./tools/build/build.sh

# Build specific component
./tools/build/build.sh kernel
./tools/build/build.sh userspace

# Run in emulator
./tools/emulator/run.sh

# Run tests
cargo test --workspace
```

## License

Aurora OS is licensed under the MIT License. See LICENSE for details.

## Contributing

We welcome contributions! Please see our contributing guidelines in CONTRIBUTING.md.

## Roadmap

See ROADMAP.md for the development timeline and milestones.
