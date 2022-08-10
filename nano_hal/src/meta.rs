use core::ptr::write_volatile;
use core::llvm_asm;

pub const F_CPU: u32 = 16000000; // 16 MHz






pub fn _BV(bit: u8) -> u8 {
    // Left shifts 1 the specfied amount
    1 << (bit)
}

pub unsafe fn set_bit(addr: *mut u8, bit: u8, state: bool) {
    // sets 'bit' at address 'addr' to 1 ('state' = true) or 0 ('state' = false)
    let val: u8 = match state {
        true => *addr | _BV(bit),
        false => *addr & !_BV(bit)
    };
    write_volatile(addr,val);
}






// Interrupts
pub unsafe fn cli() {
    llvm_asm!("cli")
}

pub unsafe fn sei() {
    llvm_asm!("sei");
}
