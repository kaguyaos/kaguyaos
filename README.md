# kaguyaos

A custom Operating System written in Rust, targeting the x86_64 UEFI architecture. This project demonstrates key OS concepts including UEFI booting, graphical framebuffer, user-mode execution, system calls, multi-tasking scheduler, device driver support (NVMe & xHCI), a custom flat filesystem, and built-in JIT compilation for both C and assembly.

![Rust](https://img.shields.io/badge/language-Rust-orange)
![Platform](https://img.shields.io/badge/platform-x86__64--UEFI-blue)

---

## ✨ Features

- **UEFI Booting**: Fully compliant with the Unified Extensible Firmware Interface standard.
- **Graphical Framebuffer**: High-resolution screen rendering.
- **User Mode (Ring 3)**: Secure transition from Kernel to User mode with Ring 3 privilege isolation.
- **Multi-tasking Scheduler**: Preemptive task scheduling supporting task yielding, termination, and exit statuses.
- **USB 3.0 Support**: Custom **xHCI Driver** supporting keyboard input with cursor/arrow-key navigation.
- **NVMe Support**: Native PCI driver for generic NVMe SSDs.
- **SimpleFS FAT Filesystem**: need update content
- **need update content**: need update content
- **need update content**: need update content
- **System Calls**: Robust 20-syscall interface for user-kernel communication.

---

## 🛠️ Prerequisites

To build and run this OS, you need the following tools installed:

- **QEMU**: For system emulation (`qemu-system-x86_64`).
- **OVMF**: UEFI firmware for QEMU.

---

## 🚀 Getting Started

### 1. Build and Run

Use the provided helper script to compile the kernel, create the disk image, and launch QEMU:

```bash
nix-shell # if you use nix
export OVMF_BIOS="/usr/share/ovmf/OVMF.fd" # if you don't use nix
./user/build.sh
./user/insert.sh
./run.sh
```

This script will:
1. Build the kernel for `x86_64-unknown-uefi`.
2. Create the necessary EFI directory structure in `esp/`.
3. Create a raw 1GB NVMe disk image (`nvme.img`) if it doesn't exist.
4. build init.kef
5. insert init.kef to nvme.img
6. Launch QEMU with the OS, USB keyboard, and NVMe drive attached.

### 2. Interactive Shell Commands

need update content

---

## 💾 FAT Filesystem

need update content

---

## 🛠️ Tiny C Compiler (`cc`)

The JIT compiler allows you to write C-like source files (normally `.c` extension) and compile/run them as Ring 3 userspace processes. It parses tokens, builds function mappings, and generates raw x86_64 machine instructions.

### 📝 Language Syntax & Limitations
- **Types**: Supports `uint64_t` and `char` (both compile as 64-bit unsigned integers). No pointer syntax or structs are supported yet.
- **Parameters**: Supports functions with up to 6 parameters (mapped to register registers `rdi`, `rsi`, `rdx`, `rcx`, `r8`, `r9` per the **System V AMD64 ABI**).
- **Operators**: No direct arithmetic operators (e.g. `+`, `-`). Computations must be done via inline assembly or helper functions.
- **Control Flow**: No loops or conditional statements (no `if`, `while`, `for`, etc.).
- **Inline Assembly**: Embed raw instructions via `asm("assembly")` or `__asm__("assembly")`. Statements inside the string can be separated by `;` or newlines.
- **Comments**: Comments (`//` or `/* */`) are currently not parsed.

### 🚀 Compilation and Execution Example
need update content

---

## 🔌 System Calls

need update content

### README.md has not been updated yet.