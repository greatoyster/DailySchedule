# Lab3代码分析

`File Organization`

The file organization is same as lab 2, the several different things is the `config.rs`, in which the mapping offset `0xffff_ffff_0000_0000` is added, related operations about kernel mapping is implemented in `entry.asm`. A new module name `mapping` is added too.

By the way, in our linker script, we do a alignment there.

`File Description`

- `address.rs`: most implementations of physical and virtual address and pages. Here it implemented a macro `implement_address_to_page_number!` about the conversion of page and address.
- 

