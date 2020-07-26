# Lab3报告

`File Organization`

The file organization is same as lab 2, the several different things is the `config.rs`, in which the mapping offset `0xffff_ffff_0000_0000` is added, related operations about kernel mapping is implemented in `entry.asm`. A new module name `mapping` is added too.

By the way, in our linker script, we do a alignment there.

`File Description`

- `address.rs`: most implementations of physical and virtual address and pages. Here it implemented a macro `implement_address_to_page_number!` about the conversion of page and address.
-  `page_table_entry.rs`: this is implementation of PTE, which is a `struct` bind with `usize`.
-  `page_table.rs`: this file defines structure  page table, which can be made up with 512(which is PAGE_SIZE/8) PTEs. Method `zero_init()` is implemented. A smaller pointer `PageTableTracker` is also implemented.
-  `segment.rs`: this file define two mapping type in a enum, and derive a struct named `Segment`, which has map type, range and flags.
-  `memory_set.rs` and `mapping.rs` is detailed implementation of mapping.

`practice`

1. Q1: in `entry.asm`,

   you can see the line 3 to line 6, we can see t0 is assigned by the address of boot page table, then it was substracted by `0xffffffff00000000`, a linear mapping offset, then it shift right by 12 which is 4KB offset, so t0 can be represented the physical page number.
   
   And from line 23, we can see the detailed layout of the page table.
   
   ```assembly
   ......
       # 通过线性映射关系计算 boot_page_table 的物理页号
       lui t0, %hi(boot_page_table)
       li t1, 0xffffffff00000000
       sub t0, t0, t1
       srli t0, t0, 12
       # 8 << 60 是 satp 中使用 Sv39 模式的记号
       li t1, (8 << 60)
       or t0, t0, t1
       # 写入 satp 并更新 TLB
       csrw satp, t0
       sfence.vma
   
       # 加载栈的虚拟地址
       lui sp, %hi(boot_stack_top)
       addi sp, sp, %lo(boot_stack_top)
       # 跳转至 rust_main
       ......
       # 初始内核映射所用的页表
       .section .data
       .align 12
       .global boot_page_table
   boot_page_table:
       # .8byte表示长度为8个字节的整数
       .8byte 0
       .8byte 0
       # 第 2 项：0x8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
       .8byte (0x80000 << 10) | 0xcf
       .zero 505 * 8
       # 第 508 项（外设用）：0xffff_ffff_0000_0000 -> 0x0000_0000，0xcf 表示 VRWXAD 均为 1
       .8byte (0x00000 << 10) | 0xcf
       .8byte 0
       # 第 510 项：0xffff_ffff_8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .8byte (0x80000 << 10) | 0xcf
       .8byte 0
   ```
   
   We can go to `rust_main` by 
   
   ```assembly
   	lui t0, %hi(rust_main)
       addi t0, t0, %lo(rust_main)
       jr t0
   ```

2. Q2: `page_tables` means all page tables, `mapped_pairs` is what we used(Maybe they can be  simply justified by their name.)
3. NO, we establish such mapping relation by modifying page table. Just through its physical address for our linear mapped kernel.