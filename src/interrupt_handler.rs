use crate::uart;

// fn decode_exception_class(exception_class: u64)

#[unsafe(no_mangle)]
pub extern "C" fn interrupt_handler(esr: u64, far: u64) -> ! {
    uart::write_reg("esr", esr);
    let exception_class = (esr >> 26) & 0x3F;
    let data_fault_status_code = (esr) & 0x3F;
    uart::write_str("exception class: ");
    uart::print_hex_u64(exception_class);
    uart::write_str("\n");
    uart::write_str("data fault status code: ");
    uart::print_hex_u64(data_fault_status_code);
    uart::write_str("\n");
    uart::write_reg("far", far);
    loop {}
}
