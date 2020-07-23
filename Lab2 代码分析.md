# Lab2 代码分析

`File Orgnization`

Compared with previous lab 1, lab 2 add two individual module `memory` and `algorithm`.

```shell
src
├── algorithm
│   ├── Cargo.toml
│   └── src
│       ├── allocator
│       │   ├── bitmap_vector_allocator.rs
│       │   ├── mod.rs
│       │   └── stacked_allocator.rs
│       └── lib.rs
├── console.rs
├── entry.asm
├── interrupt
│   ├── context.rs
│   ├── handler.rs
│   ├── interrupt.asm
│   ├── mod.rs
│   └── timer.rs
├── linker.ld
├── main.rs
├── memory
│   ├── address.rs
│   ├── config.rs
│   ├── frame
│   │   ├── allocator.rs
│   │   ├── frame_tracker.rs
│   │   └── mod.rs
│   ├── heap2.rs
│   ├── heap.rs
│   ├── mod.rs
│   └── range.rs
├── panic.rs
└── sbi.rs
```

`File Descriptions`

1. `memory`
   - `config.rs`: you can see some global configurations are declared here, such as 4K page size, the address space of memory, and the kernel heap size. Note that we use `lazy_static!` macro to initial the kernel end address before it is used in runtime. (Rust has a lot cost when implementing such concepts)
   - `address.rs`: it contains the basic implementation of physical address concept in OS, maybe in lab 3, the struct `PhysicalAddress` bind a `usize`, which is a unit struct. it have method `page_offset()` to get offset in physical page. Here it also implemented a useful marco to cover all `usize` operations, just like operator override in CXX.
   - `heap.rs`: just a simple implementation for heap, given base address and range.
   - `range.rs`: it provides us with a convinient interface of iterator in those type related to address.
   - `frame/allocator.rs`: this part is about allocator
2. `algorithm`: its submodule `allocator` provides us with a bit map allocater and stacked allocater. 

`Practice`

- Q1: we store those uninitialized global variables in bss segment, its full name is block started by symbol. The heap is accually uninitailzed data and it is seen as a global varible in our os implementation.
- Q2: it is somehow like a chicken-egg problem, such code will not work.
- Q3.1: space O(n), time O(1)
- Q3.2: implementing...