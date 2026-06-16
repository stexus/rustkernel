ENTRY(_start)
SECTIONS
{
    . = 0x40000000;

    .text : {
        *(.text._start)
        . = ALIGN(2048);
        *(.text.vbar)
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    . = ALIGN(8);
    .bss : {
        __bss_start = .;
        *(.bss*)
        . = ALIGN(8);
        __bss_end = .;
    }
    . = ALIGN(16);
    . += 0x4000;
    __sp_el2_top = .;
    . += 0x4000;
    __sp_el1_top = .;
}
