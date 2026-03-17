#![no_std]

#[cfg(target_arch = "riscv64")]
const UART_BASE: usize = 0x1000_0000;
#[cfg(target_arch = "riscv64")]
const UART_THR: usize = 0x00;
#[cfg(target_arch = "riscv64")]
const UART_LSR: usize = 0x05;
#[cfg(target_arch = "riscv64")]
const UART_LSR_THRE: u8 = 1 << 5;

#[cfg(target_arch = "riscv64")]
#[inline]
fn read_reg(offset: usize) -> u8 {
    unsafe { core::ptr::read_volatile((UART_BASE + offset) as *const u8) }
}

#[cfg(target_arch = "riscv64")]
#[inline]
fn write_reg(offset: usize, value: u8) {
    unsafe { core::ptr::write_volatile((UART_BASE + offset) as *mut u8, value) }
}

#[cfg(target_arch = "riscv64")]
pub fn uart_putc(byte: u8) {
    while (read_reg(UART_LSR) & UART_LSR_THRE) == 0 {}
    write_reg(UART_THR, byte);
}

#[cfg(not(target_arch = "riscv64"))]
pub fn uart_putc(_byte: u8) {}

pub fn uart_puts(bytes: &[u8]) {
    for &byte in bytes {
        if byte == b'\n' {
            uart_putc(b'\r');
        }
        uart_putc(byte);
    }
}
