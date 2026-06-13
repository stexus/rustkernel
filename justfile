kernel := "target/aarch64-unknown-none/debug/rustkernel"

qemu_base := "qemu-system-aarch64 \
  -machine virt,virtualization=on \
  -cpu cortex-a72 \
  -m 128M \
  -nographic \
  -serial mon:stdio \
  -kernel " + kernel

# Build and run the kernel
run: build
    {{qemu_base}}

# Build and run the kernel, paused waiting for a GDB connection on localhost:1234
debug: build
    {{qemu_base}} -s -S

# Build the kernel
build:
    cargo build
clean:
    cargo clean
