# 1. Kernel test harness: one image per test, harness owns the verdict

Date: 2026-06-16

## Status

Accepted

## Context

The kernel has no runtime input channel — it boots and runs a fixed program.
We want a regression suite that grows with every later phase, starting with a
test for synchronous exceptions at the current exception level.

A kernel "test" is awkward because the natural unit of execution is a whole
machine boot, not a function call. Two shapes were considered:

1. **One kernel image that runs all tests in-process**, looping over test
   functions and printing per-test results.
2. **One kernel image per test**, with an external harness booting each image in
   a fresh machine and aggregating results.

Exception tests deliberately fault. If a recovery is broken the machine hangs or
triple-faults. Under shape (1) the first broken recovery kills the runner and
every subsequent result is lost; tests also leak machine state (MMU, registers,
vector table) into one another.

## Decision

- **One Test Image per test.** Shared kernel code (UART, panic handler, vector
  table, `shutdown`, boot assembly) lives in `src/lib.rs`. Each test is a binary
  under `src/bin/test_*.rs` supplying its own `kernel_main`.
- **The harness owns the verdict.** `just test` boots each `test_*` image
  headless (`-display none -serial stdio -no-reboot`) under a 10s `timeout`,
  scans serial, and exits non-zero if any image fails.
- **Markers, hybrid signalling.** Images report over serial: `[PASS]` / `[FAIL]`.
  The kernel's PSCI `SYSTEM_OFF` ends each run; the harness decides the exit
  code. Semihosting exit was rejected to keep human-readable failure output.
- **Fail-closed verdict precedence:** `[FAIL]` → timeout → `[PASS]` → (no marker
  = fail). Anything ambiguous is a failure, never a silent green.
- **The panic handler emits `[FAIL]` then shuts down** (previously it spun
  forever, indistinguishable from a hang).

## Consequences

- Adding a test is dropping a file in `src/bin/`; the harness discovers it.
- `just run NAME` / `just debug NAME` boot a single named image interactively.
- A broken recovery surfaces as a per-image timeout, not a lost suite.

### Known limitation: test-specific recovery lives in the shared handler

The first synchronous-exception test needs the handler to perform Round-trip
Recovery (verify the expected ESR/FAR, advance `ELR_EL1`, return). For now that
recovery logic lives in the *shared* library handler, because there is only one
exception test.

This does not generalise: a second exception test that expects *different*
handler behaviour cannot coexist with the first, since both would share one
hard-coded handler. When that day comes, introduce a per-image installable
handler (each Test Image registers the handler it expects) and move the
recovery logic out of the shared library. Until then, the shared handler is the
recovering one.
