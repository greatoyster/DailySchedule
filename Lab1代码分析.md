# Lab1代码分析

`Code Orgnizations`

```
src
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
├── panic.rs
└── sbi.rs
```

`Related File Descriptions`

1. `sbi.rs`: a wrapper of OpenSBI, inline assembly are called by function call `sbi_call`, those machine-level functions are most low-level implementation of this lab.
2. `context.rs`: this file contains the implementation of `struct Context` which used in handling interrupts. It contains the status of registers and execution program counter. Related debug trait is also implemented.
3. `timer.rs`: interrupt in a fixed time interval. 

`Pratice`

- Q1: To handle the interrupt, the stack pointer are decreased by 34 * 8, after that it is increased by 34 * 8.

- Q2: If we remove this part we will get output like 

  ```
  mod interrupt initialized
  Hello, rCore-Tutorial!
  Breakpoint at 0x802015ac
  src/interrupt/handler.rs:59: 'Unresolved interrupt: Exception(LoadFault)
  Context { registers: [3, 80200044, 80216570, 0, 8001ce00, 80200000, 1, 1, 8001cd90, 8001ce00, 0, 3f, 3f, 0, 0, 80216394, 4, 1, 8000a628, 0, 0, 0, 0, 8, 2000, 0, 0, 0, 0, 0, 0, 82200000], sstatus: Sstatus { bits: 8000000000006120 }, sepc: 80201a42 }
  stval: 0'
  ```

  The thread will be terminated for fault in QEMU.

- Q3.1: in the match clause of `handler::handle_interrupt(...)`, we just need to add an arm `Trap::Exception(Exception::LoadFault)=>panic!()`

- Q3.2: in `handler::handle_interrupt(...)`, just add while handling such exceptions.

  ```rust
  if stvel == 0x0{
      println!("Success");
  }
  ```

- Q3.3: we can add jump instructions in assembly to access memory address 0x0.

