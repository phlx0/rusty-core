# rusty-core 🦀

A minimal **x86_64 Rust kernel** built with `no_std`, using the legacy
`bootloader + cargo-bootimage` workflow.  
This project boots in **QEMU** and prints output via **serial (COM1)**.

This repository is intended for **learning operating system development in Rust**.

---

## ✨ Features

- `#![no_std]`, `#![no_main]` kernel
- Custom `x86_64` target (`x86_64.json`)
- Boots via legacy BIOS bootloader
- Serial output (works in WSL / headless environments)
- Runs in QEMU
- Written entirely in Rust

---

## 🧰 Requirements

You need the following installed:

### 1. Rust (nightly)

```bash
rustup install nightly
rustup default nightly
```

### 2. Required Rust components

```bash
rustup component add llvm-tools-preview
```

### 3. Cargo tools

```bash
cargo install bootimage
```

### 4. QEMU

On Ubuntu / WSL:

```bash
sudo apt install qemu-system-x86
```

---

## ▶️ Building the Kernel

From the project root:

```bash
cargo bootimage
```

If successful, this will create:

```
target/x86_64/debug/bootimage-rusty-core.bin
```

---

## ▶️ Running the Kernel

Run QEMU manually (recommended, especially on WSL):

```bash
qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64/debug/bootimage-rusty-core.bin \
  -nographic
```

You should see:

```
Hello from the kernel!
It is running :)
```

To exit QEMU:

```
Ctrl + A, then X
```

---

## 🖥️ Platform Notes

### ✅ Works on

- Linux
- WSL (Windows Subsystem for Linux)
- macOS (with QEMU installed)

### ⚠️ Notes

- This is a **BIOS-based** bootloader (legacy workflow)
- Uses serial output, not VGA or framebuffer (yet)
- Not intended for real hardware

---

## 📁 Project Structure

```
.
├── src/main.rs          # Kernel entry point
├── Cargo.toml
├── .cargo/config.toml   # Custom target + runner
├── x86_64.json          # Target specification
└── README.md
```

---

## 🚧 Roadmap

Planned next steps:

- VGA text mode output
- Interrupts & timers
- Heap allocator
- Paging / higher-half kernel
- Keyboard input
