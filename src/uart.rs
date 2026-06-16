// PL011 UART driver for QEMU virt machine
// Base address: 0x0900_0000

const UART_BASE: *mut u32 = 0x0900_0000 as *mut u32;

// Register offsets (byte offset / 4 since we use *mut u32)
const DR: usize = 0x000 / 4; // Data Register
const FR: usize = 0x018 / 4; // Flag Register

// Flag Register bits
const FR_TXFF: u32 = 1 << 5; // Transmit FIFO Full
const FR_RXFE: u32 = 1 << 4; // Receive FIFO Empty

/// Write a single byte to the UART
pub fn write_byte(byte: u8) {
    // transmit is full
    unsafe {
        let dr_addr = UART_BASE.add(DR);
        let fr_addr = UART_BASE.add(FR);
        while (core::ptr::read_volatile(fr_addr) & FR_TXFF) != 0 {}
        core::ptr::write_volatile(dr_addr, byte as u32);
    }
}

/// Read a single byte from the UART (blocks until one is available)
pub fn read_byte() -> u8 {
    unsafe {
        let dr_addr = UART_BASE.add(DR);
        let fr_addr = UART_BASE.add(FR);
        while (core::ptr::read_volatile(fr_addr) & FR_RXFE) != 0 {}
        core::ptr::read_volatile(dr_addr) as u8
    }
}
pub fn write_str(str: &str) {
    for b in str.bytes() {
        write_byte(b);
    }
}
/// Write a u64 value to the UART as a zero-padded hex string, e.g. `0x000000004008F000`.
pub fn print_hex_u64(val: u64) {
    const HEX_DIGITS: &[u8] = b"0123456789ABCDEF";
    write_byte(b'0');
    write_byte(b'x');
    // Emit all 16 nibbles, most-significant first.
    for shift in (0..16).rev() {
        let nibble = ((val >> (shift * 4)) & 0xF) as usize;
        write_byte(HEX_DIGITS[nibble]);
    }
}

/// Write a labelled register value to the UART as a human-readable string.
///
/// Output format: `"<label>: 0x<16-digit hex>\r\n"`
/// e.g. `write_reg("PC", 0x4008_F000)` → `PC: 0x000000004008F000`
pub fn write_reg(label: &str, reg: u64) {
    write_str(label);
    write_str(": ");
    print_hex_u64(reg);
    write_str("\r\n");
}
