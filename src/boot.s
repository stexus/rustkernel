.section .text._start
.global _start
_start:
msr SPSel, #1

ldr x0, =__sp_el2_top
mov sp, x0
ldr x0, =__sp_el1_top
msr SP_EL1, x0

// starting at __bss_start, loop through all bytes and zero them
ldr x0, =__bss_start
ldr x1, =__bss_end
bss_clear:
  cmp x0, x1
  b.ge bss_clear_done
  str xzr, [x0], #8
  b bss_clear
bss_clear_done:

# set return address to kernel_main
ldr x0, =kernel_main
msr elr_el2, x0

# set bit 31 of hcr (hcr.rw)
mrs x0, hcr_el2
orr x0, x0, (1<<31)
msr hcr_el2, x0

# set spsr to prepare for correct pstate in el1
mov x0, #0x3c5
msr spsr_el2, x0

eret

