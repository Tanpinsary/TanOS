# TanOS Rust 学习路线与 OS 实现计划

## 一、目标概述

TanOS 是一个基于 rCore 思路实现的 RISC-V 简易操作系统，作为操作系统大作业使用。项目目标是在 QEMU RISC-V 虚拟机中完成内核启动，并实现至少一个可展示功能。

推荐最终目标：

- 在 QEMU RISC-V 虚拟机中启动 TanOS。
- 启动后输出预设欢迎字符串。
- 支持基础控制台输出。
- 输出内核内存布局。
- 实现物理页帧分配器。
- 开启 Sv39 分页机制。
- 实现简单内核堆分配测试。

推荐启动输出示例：

```text
Welcome to TanOS!
[TanOS] booting on RISC-V QEMU virt machine
[TanOS] console initialized
[TanOS] kernel memory layout:
[TanOS] .text   = ...
[TanOS] .rodata = ...
[TanOS] .data   = ...
[TanOS] .bss    = ...
[TanOS] frame allocator initialized
[TanOS] paging enabled: Sv39
[TanOS] heap initialized
[TanOS] memory demo passed
[TanOS] kernel boot completed
```

## 二、Rust 学习路线

### 1. Rust 基础语法

先掌握写普通 Rust 程序的能力。

需要学习：

- 变量与可变性。
- 常量。
- 基本数据类型。
- `if` / `loop` / `while` / `for`。
- 函数。
- 模块系统。
- `struct`。
- `enum`。
- `match`。
- `impl`。

学习目标：能看懂并写出基础 Rust 代码。

### 2. 所有权与生命周期

这是 Rust 写 OS 的核心基础。

需要学习：

- 所有权 ownership。
- 借用 borrowing。
- 不可变引用 `&T`。
- 可变引用 `&mut T`。
- 生命周期 lifetime。
- 移动语义 move。
- 拷贝语义 copy。
- `Clone` 与 `Copy`。

学习目标：理解 Rust 为什么不需要 GC，也能保证内存安全。

### 3. 常用标准库能力

虽然写内核时不能依赖完整标准库，但学习阶段仍然需要掌握常见 Rust 数据结构和错误处理方式。

需要学习：

- `String`。
- `Vec`。
- `Option`。
- `Result`。
- `Box`。
- `Rc`。
- `Arc`。
- `Mutex`。
- `HashMap`。
- 错误处理。

学习目标：理解 Rust 常见数据结构和错误处理习惯。

### 4. 泛型、Trait 与迭代器

rCore 代码中会大量出现这些内容。

需要学习：

- 泛型函数。
- 泛型结构体。
- `trait`。
- trait bound。
- `where` 约束。
- `Iterator`。
- 闭包 closure。
- `Drop`。

学习目标：能看懂 rCore 中较抽象的代码封装。

### 5. Unsafe Rust

操作系统内核不可避免需要使用 `unsafe`。

需要学习：

- 裸指针 `*const T` / `*mut T`。
- 指针读写。
- `unsafe fn`。
- `unsafe block`。
- 内联汇编 `asm!`。
- volatile 读写。
- 全局静态变量 `static mut`。

学习目标：知道哪些地方必须使用 `unsafe`，并学会把 `unsafe` 限制在尽量小的范围内。

### 6. no_std Rust

操作系统内核不能依赖普通操作系统提供的标准库，所以需要学习裸机 Rust 编程。

需要学习：

- `#![no_std]`。
- `#![no_main]`。
- panic handler。
- linker script。
- 自定义入口函数。
- 裸机目标平台。
- 交叉编译。

学习目标：理解 Rust 程序如何在没有操作系统的环境下运行。

## 三、OS 基础学习路线

### 1. 计算机启动流程

需要学习：

- CPU 上电后从哪里开始执行。
- BIOS/UEFI 与裸机启动的区别。
- RISC-V 的启动流程。
- OpenSBI 的作用。
- QEMU virt 机器。
- 内核入口地址。
- 链接脚本如何控制内核加载位置。

学习目标：理解 TanOS 是如何被 QEMU 启动起来的。

### 2. RISC-V 基础

需要学习：

- RISC-V 常用寄存器。
- 特权级：M-mode、S-mode、U-mode。
- 常用 CSR 寄存器。
- 异常与中断。
- `ecall`。
- SBI 调用。
- Sv39 分页机制。

学习目标：理解 rCore 为什么通过 SBI 输出字符，以及为什么要切换页表。

### 3. 内存管理

这是 TanOS 推荐重点实现的大作业功能。

需要学习：

- 物理内存布局。
- 栈。
- 堆。
- 页。
- 页表。
- 虚拟地址。
- 物理地址。
- 地址映射。
- 页帧分配器。
- 内核堆分配器。

学习目标：能实现开启分页、页帧分配和简单堆分配测试。

### 4. 进程与线程基础

如果后续想扩展 TanOS，可以继续学习进程与线程。

需要学习：

- 任务上下文。
- 上下文切换。
- trap 机制。
- 系统调用。
- 进程地址空间。
- 调度器。
- 线程与锁。

学习目标：知道进程和线程操作的基本思路。该部分难度较高，不建议作为第一阶段主功能。

## 四、TanOS 实现过程

### 1. 建立项目基础

建议从 `rCore-Tutorial-v3` 开始，不建议一开始完全从零写全部代码。

官方教程手册：

```text
https://rcore-os.cn/rCore-Tutorial-Book-v3/
```

官方代码仓库：

```text
https://github.com/rcore-os/rCore-Tutorial-v3
```

第一步目标：

- fork 或复制 rCore 教程仓库。
- 改名为 `TanOS`。
- 保留 `os/` 作为内核目录。
- 跑通 `make run`。

期望结果：能看到 rCore 原始启动输出。

### 2. 改造成 TanOS

需要修改：

- 项目名称。
- 启动 banner。
- README。
- Makefile 中的名字。
- 启动输出字符串。

示例输出：

```text
Welcome to TanOS!
TanOS is a tiny RISC-V operating system based on rCore.
```

这一阶段可以作为最基础验收功能。

### 3. 实现控制台输出

整理或实现 console 模块。

需要实现：

- `console_putchar`。
- `print!`。
- `println!`。
- `log_info!`。
- `log_warn!`。
- `log_error!`。

目标输出：

```text
[TanOS] kernel started
[TanOS] console initialized
[TanOS] boot sequence begin
```

### 4. 实现内存布局输出

通过链接脚本提供的符号，输出内核各段地址。

需要输出：

- `.text` 段地址。
- `.rodata` 段地址。
- `.data` 段地址。
- `.bss` 段地址。
- kernel end 地址。

示例输出：

```text
[TanOS] text section: ...
[TanOS] rodata section: ...
[TanOS] data section: ...
[TanOS] bss section: ...
[TanOS] kernel end: ...
```

这个功能实现难度较低，但很适合展示对内核内存布局的理解。

### 5. 实现物理页帧分配器

核心内容：

- 定义 `PhysAddr`。
- 定义 `PhysPageNum`。
- 定义 `FrameTracker`。
- 定义 `FrameAllocator`。
- 初始化可用物理页范围。
- 支持 `alloc`。
- 支持 `dealloc`。

测试输出：

```text
[TanOS] frame allocator initialized
[TanOS] alloc frame 1: 0x80210000
[TanOS] alloc frame 2: 0x80211000
[TanOS] dealloc frame successfully
```

### 6. 实现页表与分页机制

核心内容：

- 学习 Sv39 页表格式。
- 实现页表项 `PageTableEntry`。
- 实现三级页表。
- 建立内核地址空间。
- 映射内核代码段。
- 映射内核数据段。
- 映射 MMIO 区域。
- 写入 `satp`。
- 执行 `sfence.vma`。

目标输出：

```text
[TanOS] kernel address space initialized
[TanOS] paging enabled: Sv39
```

这是大作业中最有含金量的功能之一。

### 7. 实现内核堆分配

可以参考 rCore 的实现方式，引入 allocator。

需要实现：

- 配置 kernel heap。
- 初始化堆空间。
- 测试 `Box`。
- 测试 `Vec`。

测试输出：

```text
[TanOS] heap initialized
[TanOS] Box test passed
[TanOS] Vec test passed
```

### 8. 实现 TanOS 内存管理演示函数

为了答辩和验收展示，可以写一个固定演示函数，在启动时自动调用。

示例：

```rust
fn tanos_memory_demo() {
    println!("[TanOS] memory demo begin");
    // 输出内存布局
    // 分配页帧
    // 释放页帧
    // 测试 Box / Vec
    println!("[TanOS] memory demo passed");
}
```

### 9. 可选扩展：任务与进程演示

如果后续还有精力，可以继续扩展任务和进程功能。

可选内容：

- 加载一个用户程序。
- trap 处理。
- 简单系统调用。
- 多任务轮转。
- 输出任务切换日志。

示例输出：

```text
[TanOS] task 0 running
[TanOS] task 1 running
[TanOS] switching task 0 -> task 1
```

该部分比内存管理更难，建议作为扩展功能，而不是第一阶段主目标。

## 五、推荐最终功能组合

最稳的大作业版本：

- QEMU RISC-V 虚拟机启动 TanOS。
- 启动后输出预设欢迎字符串。
- 支持 `println!`。
- 输出内核内存布局。
- 实现物理页帧分配器。
- 开启 Sv39 分页机制。
- 实现简单内核堆分配测试。

这个版本比只打印字符串更完整，但又不会像实现完整进程系统那样失控。

## 六、建议验收材料

最终可以准备以下材料：

- TanOS 项目源码。
- QEMU 启动截图。
- 启动日志截图。
- 内存布局输出截图。
- 页帧分配测试截图。
- 分页机制开启日志截图。
- 简短设计说明文档。

答辩时可以重点说明：

- TanOS 如何从 QEMU 启动。
- OpenSBI 在启动中的作用。
- `no_std` Rust 内核和普通 Rust 程序的区别。
- 内核如何输出字符。
- 物理页帧如何分配和释放。
- Sv39 分页机制如何开启。
