/* link.ld */
ENTRY(_start)
OUTPUT_ARCH(riscv)

/* 定義基地址，應用程式將被加載到這個地址 */
BASE_ADDRESS = 0xffffffc087000000;

/* 定義頁對齊大小 */
PAGE_SIZE = 4K;

SECTIONS
{
  /* Load the app at this address: "." means the current address */
  . = BASE_ADDRESS;
  start = .;

  .text : ALIGN(4K) {
    stext = .;
    *(.text.entry)
    *(.text .text.*)
    etext = .;
  }

  .plt : ALIGN(4K) {
    splt = .;
    *(.plt .plt.*)
    eplt = .;
  }

  .rodata : ALIGN(4K) {
    srodata = .;
    *(.rodata .rodata.*)
    erodata = .;
  }

  .data : ALIGN(4K) {
    sdata = .;
    *(.data .data.*)
    edata = .;
  }

  .bss : ALIGN(4K) {
    sbss = .;
    *(.bss .bss.*)
    ebss = .;
  }

  /* Automatically set the global pointer to the start of the .data section */
  PROVIDE(__global_pointer$ = sdata);

  PROVIDE(end = .);
}
