BASE_ADDRESS = 0xffffffc080100000;

ENTRY(_start)

SECTIONS
{
    . = BASE_ADDRESS;
    .text : ALIGN(8) {
        *(.text)
    }

    .plt : ALIGN(8) {
        *(.plt)
    }

    .interp : ALIGN(8) {
        *(.interp)
    }

    .dynsym : ALIGN(8) {
        *(.dynsym)
    }

    .dynstr : ALIGN(8) {
        *(.dynstr)
    }

    .hash : ALIGN(8) {
        *(.hash)
    }

    .gnu.hash : ALIGN(8) {
        *(.gnu.hash)
    }

    .rodata : ALIGN(8) {
        *(.rodata)
    }

    .rela.dyn : ALIGN(8) {
        *(.rela.dyn)
    }

    .dynamic : ALIGN(8) {
        *(.dynamic)
    }

    .got : ALIGN(8) {
        *(.got)
    }

    .got.plt : ALIGN(8) {
        *(.got.plt)
    }

    .data : ALIGN(8) {
        *(.data)
    }

    .bss : ALIGN(8) {
        *(.bss)
    }

}