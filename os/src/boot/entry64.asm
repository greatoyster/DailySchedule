  .section .text.entry
  .globl _start
_start:
  la sp, bootstacktop
  call rust_main

  .section .bss.stack
  .align 12
  .global bootstack
bootstack:
  .space 4*4096
  .global bootstacktop
bootstacktop:
