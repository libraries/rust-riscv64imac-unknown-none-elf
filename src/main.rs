#![no_main]
#![no_std]

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    // If the main thread panics it will terminate all your threads and end your program with code 101.
    // See: https://github.com/rust-lang/rust/blob/master/library/core/src/macros/panic.md
    syscall_exit(101)
}

fn syscall(mut a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64) -> u64 {
    unsafe {
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
        )
    }
    a0
}

// Linux system calls for the RISC-V architecture: Exit.
// See: https://github.com/westerndigitalcorporation/RISC-V-Linux/blob/master/riscv-pk/pk/syscall.h
fn syscall_exit(code: u64) -> ! {
    syscall(code, 0, 0, 0, 0, 0, 0, 93);
    loop {}
}

// Linux system calls for the RISC-V architecture: Write.
// See: https://github.com/westerndigitalcorporation/RISC-V-Linux/blob/master/riscv-pk/pk/syscall.h
fn syscall_write(fd: u64, buf: *const u8, count: u64) -> u64 {
    // Stdin is defined to be fd 0, stdout is defined to be fd 1, and stderr is defined to be fd 2.
    syscall(fd, buf as u64, count, 0, 0, 0, 0, 64)
}

#[no_mangle]
pub unsafe extern "C" fn _start() {
    core::arch::asm!(
        "lw a0,0(sp)", // Argc.
        "add a1,sp,8", // Argv.
        "li a2,0",     // Envp.
        "call main",
        "li a7, 93",
        "ecall",
    );
}

#[no_mangle]
unsafe extern "C" fn main(argc: u64, argv: *const *const i8) -> u64 {
    for i in 1..argc {
        let argn = core::ffi::CStr::from_ptr(argv.add(i as usize).read());
        let argn = argn.to_bytes();
        syscall_write(1, argn.as_ptr(), argn.len() as u64);
        if i != argc - 1 {
            syscall_write(1, [32].as_ptr(), 1);
        }
    }
    syscall_write(1, [10].as_ptr(), 1);
    return 0;
}
