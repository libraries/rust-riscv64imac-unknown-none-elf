# 使用 Rust 编写 RISC-V 程序

我们将在本文中验证是否能用 Rust 编写一个 RISC-V 程序. 本文的目的是探索有关 Rust 交叉编译的功能, 并在编译 Rust 程序时为 RISC-V 提供确切的步骤和配置.

## 环境准备

为 Rust 安装 riscv64imac-unknown-none-elf target

```sh
$ rustup target add riscv64imac-unknown-none-elf
```

## 创建项目

创建一个名称为 rust-riscv64imac-unknown-none-elf 的 Rust 项目

```sh
$ cargo new --bin rust-riscv64imac-unknown-none-elf
```

为此项目指定默认编译目标. 创建 `.cargo/config.toml` 并写入以下内容

```toml
[build]
target = "riscv64imac-unknown-none-elf"
```

## 构建最小可行程序

将以下内容写入 `src/main.rs`, 这就是一个最小的可成功编译的 Rust 程序.

```rs
#![no_main]
#![no_std]

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
```

编译它, 并使用 `riscv64-unknown-elf-objdump` 检查编译结果

```sh
$ cargo build --release
$ riscv64-unknown-elf-objdump -d target/riscv64imac-unknown-none-elf/release/rust-riscv64imac-unknown-none-elf

target/riscv64imac-unknown-none-elf/release/rust-riscv64imac-unknown-none-elf:     file format elf64-littleriscv
```

结果显示它是一个空的二进制文件, 内部甚至没有一条指令.

## 添加系统调用

我们日常使用的大多数程序都会返回退出码, 提示您程序是执行成功还是失败: 通常情况下, 0 代表成功, 任意非 0 值代表程序执行失败. 现在我们要在此 Rust 程序中增加一个功能: 它永远返回 42 退出码.

将下面的代码写入 `src/main.rs`, 并重新编译.

```rs
unsafe fn syscall(
    mut a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
) -> u64 {
    core::arch::asm!(
      "ecall",
      inout("a0") a0,
      in("a1") a1,
      in("a2") a2,
      in("a3") a3,
      in("a4") a4,
      in("a5") a5,
      in("a6") a6,
      in("a7") a7
    );
    a0
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe { syscall(42, 0, 0, 0, 0, 0, 0, 93) };
    loop {}
}
```

如果我们使用 `riscv64-unknown-elf-objdump` 重新检查它, 会发现它现在已经有代码块了.

```sh
$ riscv64-unknown-elf-objdump -d target/riscv64imac-unknown-none-elf/release/rust-riscv64imac-unknown-none-elf

target/riscv64imac-unknown-none-elf/release/rust-riscv64imac-unknown-none-elf:     file format elf64-littleriscv


Disassembly of section .text:

0000000000011120 <_start>:
   11120:       05d00893                li      a7,93
   11124:       02a00513                li      a0,42
   11128:       4581                    li      a1,0
   1112a:       4601                    li      a2,0
   1112c:       4681                    li      a3,0
   1112e:       4701                    li      a4,0
   11130:       4781                    li      a5,0
   11132:       4801                    li      a6,0
   11134:       00000073                ecall
   11138:       a001                    j       11138 <_start+0x18>
```

## 执行程序

我们可以使用 QEMU 模拟器来执行编译好的程序.

```sh
$ apt install qemu
$ qemu-riscv64 target/riscv64imac-unknown-none-elf/release/rust-riscv64imac-unknown-none-elf
$ echo $?
42
```

如果我们配置好相关的环境变量, 也可以直接使用 `cargo run` 来运行程序:

```sh
$ export CARGO_TARGET_RISCV64IMAC_UNKNOWN_NONE_ELF_RUNNER=qemu-riscv64
$ cargo run
$ echo $?
42
```

如果您希望省却环境变量的配置工作, 还可以选择将信息长久写入 `.cargo/config.toml` 配置文件中.

```sh
[target.riscv64imac-unknown-none-elf]
runner = "qemu-riscv64"
```

## 重构 panic_handler

在我们先前的最小可行程序中, panic_handler 是一个死亡循环: 这不是我们希望的. 重构它, 让它以 43 退出:

```rs
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    unsafe { syscall(43, 0, 0, 0, 0, 0, 0, 93) };
}
```

## 完整代码参考

```rs
#![no_main]
#![no_std]

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    unsafe { syscall(43, 0, 0, 0, 0, 0, 0, 93) };
    loop {}
}

unsafe fn syscall(
    mut a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
) -> u64 {
    core::arch::asm!(
      "ecall",
      inout("a0") a0,
      in("a1") a1,
      in("a2") a2,
      in("a3") a3,
      in("a4") a4,
      in("a5") a5,
      in("a6") a6,
      in("a7") a7
    );
    a0
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe { syscall(42, 0, 0, 0, 0, 0, 0, 93) };
    loop {}
}
```

完整项目托管于 Github: <https://github.com/libraries/rust-riscv64imac-unknown-none-elf>

## 参考

- [1] [Build CKB contract with Rust - part 1, jjy](https://talk.nervos.org/t/build-ckb-contract-with-rust-part-1/4064)
- [2] [RISC-V Bytes: Rust Cross-Compilation, DANIEL MANGUM](https://danielmangum.com/posts/risc-v-bytes-rust-cross-compilation/)
