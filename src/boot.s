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

.align 11
.section .text.vbar
.global vbar_entry
vbar_entry:
# will need 16 sections of 128bytes each (2^7 * 2^4 = 2^11 or 2048 bytes)
.org 0x000
1:
wfe
b 1b
.org 0x080
1:
wfe
b 1b
.org 0x100
1:
wfe
b 1b
.org 0x180
1:
wfe
b 1b
.org 0x200
1:
wfe
b 1b
.org 0x280
1:
wfe
b 1b
.org 0x300
1:
wfe
b 1b
.org 0x380
1:
wfe
b 1b
.org 0x400
1:
wfe
b 1b
.org 0x480
1:
wfe
b 1b
.org 0x500
1:
wfe
b 1b
.org 0x580
1:
wfe
b 1b
.org 0x600
1:
wfe
b 1b
.org 0x680
1:
wfe
b 1b
.org 0x700
1:
wfe
b 1b
.org 0x780
1:
wfe
b 1b
