target_dir := "target/aarch64-unknown-none/debug"
default_image := "main"

qemu_base := "qemu-system-aarch64 \
  -machine virt,virtualization=on \
  -cpu cortex-a72 \
  -m 128M \
  -nographic \
  -serial mon:stdio \
  -kernel "

# Build the library and every src/bin image.
build:
    cargo build

# Run an image (default `main`), e.g. `just run test_sync_exception`.
run NAME=default_image: build
    {{qemu_base}} {{target_dir}}/{{NAME}}

# Run an image paused for GDB on :1234 (default `main`); refreshes .gdbinit symbols.
debug NAME=default_image: build
    printf 'set architecture aarch64\nfile %s\ntarget remote :1234\n' "{{target_dir}}/{{NAME}}" > .gdbinit
    {{qemu_base}} {{target_dir}}/{{NAME}} -s -S

# Regression suite: boot every test_* image headless, exit nonzero on any failure.
test: build
    #!/usr/bin/env bash
    set -uo pipefail
    failed=0
    shopt -s nullglob
    images=(src/bin/test_*.rs)
    if [ ${#images[@]} -eq 0 ]; then
        echo "no test images found in src/bin/test_*.rs"
        exit 1
    fi
    for src in "${images[@]}"; do
        name=$(basename "$src" .rs)
        bin="{{target_dir}}/$name"
        output=$(timeout 10s qemu-system-aarch64 \
            -machine virt,virtualization=on \
            -cpu cortex-a72 \
            -m 128M \
            -display none \
            -serial stdio \
            -no-reboot \
            -kernel "$bin" 2>&1)
        code=$?
        # Fail-closed: [FAIL] -> timeout -> [SKIP] -> [PASS] -> no verdict.
        if echo "$output" | grep -q '\[FAIL\]'; then
            echo "FAIL $name"
            echo "$output" | sed 's/^/    /'
            failed=1
        elif [ $code -eq 124 ]; then
            echo "FAIL $name (timeout)"
            failed=1
        elif echo "$output" | grep -q '\[SKIP\]'; then
            echo "SKIP $name (not implemented)"
        elif echo "$output" | grep -q '\[PASS\]'; then
            echo "PASS $name"
        else
            echo "FAIL $name (no verdict)"
            echo "$output" | sed 's/^/    /'
            failed=1
        fi
    done
    exit $failed

clean:
    cargo clean
