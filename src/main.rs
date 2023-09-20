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
