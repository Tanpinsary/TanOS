# TanOS

TanOS is a RISC-V teaching operating system based on the official `rCore-Tutorial-v3` project. It is used as an operating system course project and runs on the QEMU RISC-V virtual machine.

## Upstream

TanOS is forked from:

```text
https://github.com/rcore-os/rCore-Tutorial-v3
```

Official rCore tutorial book:

```text
https://rcore-os.cn/rCore-Tutorial-Book-v3/
```

## Project Goal

The first TanOS target is to complete a bootable RISC-V kernel and provide visible course-project features:

- Boot TanOS in QEMU RISC-V.
- Print a TanOS welcome banner after startup.
- Keep the rCore-style console, memory, task, file system, and driver structure as the learning base.
- Extend TanOS with self-designed features such as memory layout output, frame allocation tests, paging demos, or process/thread demos.

## Run

From the project root, run QEMU:

```sh
make run
```

You can also enter the kernel directory directly:

```sh
cd os
make run
```

Expected early output includes:

```text
Welcome to TanOS!
TanOS is a tiny RISC-V operating system based on rCore.
[TanOS] booting on RISC-V QEMU virt machine
```

To exit QEMU, press `Ctrl+a` and then `x`.

After TanOS reaches the shell, run the memory information demo:

```sh
meminfo
```

The command prints the managed physical memory range, frame allocator usage,
kernel heap usage, and current paging mode.

## Directory Layout

```text
TanOS/
  os/              Kernel code
  user/            User programs
  easy-fs/         File system library
  easy-fs-fuse/    File system image builder
  bootloader/      Bootloader images
  figures/         Tutorial figures inherited from rCore
```

## License

TanOS keeps the upstream rCore-Tutorial-v3 license. See `LICENSE` for details.
