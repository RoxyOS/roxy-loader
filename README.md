# roxy-loader

Lightweight Rust bootloader for kernel development.

## Quick Start

If you want to build your own kernel on top of `roxy-loader`, start from the [template repository](https://github.com/RoxyOS/roxy-loader-template):

If you are adapting an existing kernel to use `roxy-loader`, you can use the template repository as a reference.

For more information, check the [README](https://github.com/RoxyOS/roxy-loader-template/blob/main/README.md) of the template repository

## C Support

If you want to use `roxy-loader` with a C kernel, you can refer to the
[C template](https://github.com/RoxyOS/roxy-loader-c-template).

> [!WARNING]
> Although `roxy-loader` officially supports C kernels, it is not recommended.
> `roxy-loader` has the best support for Rust kernels.

## Interface

This section assumes your kernel follows the same structure as the template repository, or that you are using the template directly.

### Framebuffer

The framebuffer is available as `bootinfo.framebuffer`.

It currently provides:

- `ptr()`: returns the framebuffer base pointer as `*mut u8`
- `size`: total framebuffer size in bytes
- `stride`: framebuffer stride

Example:

```rust
unsafe {
    let ptr = bootinfo.framebuffer.ptr();

    for i in 0..bootinfo.framebuffer.size {
        ptr.add(i).write_volatile(0);
    }
}
```

### Memory Map

Memory map access is not supported yet.

### SMP

`roxy-loader` does not support SMP.

If you need SMP, use the [`ap-startup`](https://crates.io/crates/ap-startup) crate.

## Developing roxy-loader

This repository itself is the loader implementation and its supporting crates.

### Requirements

- Rust toolchain from `rust-toolchain.toml`
- `qemu-system-x86_64`
- optionally `nix` if you want to use the provided dev shell

Enter the dev shell with Nix:

```bash
nix develop
```

Or use your local Rust + QEMU installation directly.

### Commands

Build and run the basic kernel for quick testing in QEMU:

```bash
cargo xrun-basic-kernel
```

Run host-side and kernel-side tests:

```bash
cargo xtest
```
