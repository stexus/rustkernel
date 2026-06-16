.macro restore_reg_state
  ldp x0,  x1,  [sp, #0]
  ldp x2,  x3,  [sp, #16]
  ldp x4,  x5,  [sp, #32]
  ldp x6,  x7,  [sp, #48]
  ldp x8,  x9,  [sp, #64]
  ldp x10, x11, [sp, #80]
  ldp x12, x13, [sp, #96]
  ldp x14, x15, [sp, #112]
  ldp x16, x17, [sp, #128]
  ldp x18, x19, [sp, #144]
  ldp x20, x21, [sp, #160]
  ldp x22, x23, [sp, #176]
  ldp x24, x25, [sp, #192]
  ldp x26, x27, [sp, #208]
  ldp x28, x29, [sp, #224]
  ldr x30,      [sp, #240]
  add sp, sp, #256
.endmacro

.macro save_reg_state
  sub sp, sp, #256
  stp x0,  x1,  [sp, #0]
  stp x2,  x3,  [sp, #16]
  stp x4,  x5,  [sp, #32]
  stp x6,  x7,  [sp, #48]
  stp x8,  x9,  [sp, #64]
  stp x10, x11, [sp, #80]
  stp x12, x13, [sp, #96]
  stp x14, x15, [sp, #112]
  stp x16, x17, [sp, #128]
  stp x18, x19, [sp, #144]
  stp x20, x21, [sp, #160]
  stp x22, x23, [sp, #176]
  stp x24, x25, [sp, #192]
  stp x26, x27, [sp, #208]
  stp x28, x29, [sp, #224]
  str x30,      [sp, #240]
.endmacro
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
# 0x000 is curr el, sp0
# 0x200 is curr el, curr sp
# 0x400 is lower el, aarch64
# 0x600 is lower el, aarch32
# within each section, we have synchronous, irq, frq, serror
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
# curr el, curr sp, synchronous
b sync_handler_el1

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

sync_handler_el1:
  save_reg_state
  mrs x0, esr_el1
  mrs x1, far_el1
  bl interrupt_handler
  restore_reg_state
  eret

