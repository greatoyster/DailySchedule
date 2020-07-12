OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x80200000;

SECTIONS{
  .text : {
    . = BASE_ADDRESS;
    stext = .;
    *(.text_entry)
    *(.text .text.*)
    . = ALIGN(4K)
    etext = .;
  }
  .rodata : {
    srodata = .;
    *(.rodata .rodata.*)
    . = ALIGN(4K);
    erodata = .;
  }
  .data : {
    sdata = .;
    *(.data .data.*)
    edata = .;
  }
  .stack : {
    sstack = .;
    *(.stack .stack.*)
    estack = .;
  }
  .bss : {
    sbss = .;
    *(.bss .bss.*)
    ebss = .;
  }
  PROVIDE(end = .);
}
