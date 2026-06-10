# Tell GDB we're debugging an AArch64 target
set architecture aarch64

# Load the kernel ELF (gives us symbols, source lines, function names)
file target/aarch64-unknown-none/debug/rustkernel

# Connect to QEMU's GDB stub (opened by -s flag)
target remote :1234

# Break at the first Rust entry point
break kernel_main

# Let it run until the breakpoint
continue
