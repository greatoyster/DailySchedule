 

# DailySchedule

Daily schedule of OS Tutorial Summer of Code 2020

Here is the links of [rCore-Tutorial-issues](https://github.com/rcore-os/rCore-Tutorial/issues/)

# Day0

Today I applied for this project and admitted later. The tutor told me that I'd better take it step by step. To record my daily schedule, I create this repo.


The first task is to learn Rust programming language. I plan to finished the first part before 5 July 2020. Before 3 July, I have to prepare my final exams, So I will spend 2 hours per day during this period. 

# Day1

Today I planed do a revision on Rust programming language.

Luckily, the material I have learned by myself before, and I wrote a summary of those books. You can refer to them [Rust Summary](https://greatoyster.github.io/2020/02/01/Rust%E5%85%A5%E9%97%A8%E8%AF%AD%E6%B3%95%E5%BD%92%E7%BA%B3/#more)

And there are some small exercises in rustlings, as requested, I also integrated them in this repo folder `./rustlings`.

It seems some error while installation, I have to run  `rustup update`.

This morning, I spent about 2 hours to finish 50% of `rustlings`. 

# Day2

Today I kept learning Rust, and I found I am weak at part of error handling, I will spend more time one that.

As planed, the updated codes are contained in `rustling-exercise` folder.

# Day3

Today I updated my notes about error handling and trait, you can found it it previous link.

In fact, the polymorphism of Rust is somehow a little different from that of C-like languages.

# Day4

Toady I finished all of rustlings, after this comprehensive practice, I have better grasp of Rust.

What I need to do next is to get familiar with coming rCore-Tutorial, especially the OS related part. 

# Day5

Today I would like to experience `grpc-rs`. I spent the whole morning and afternoon to implemented the main functionalities of our previous class project --- RPC-based distributed file system.

I first defined the interface of using `protobuf`, then I generated the basic framework of the client and server. Step by step, it is smoother than our previous implementation in C language. Compared with C, Rust has less crashes in run time. In addition, the speed of developing is not so slow for the familiarity of  Rust. The new feature is that the Rust version server now support multi threads, I implemented a thread pool for the server.

In the rest of today, I learned some OS lectures,

# Day6

Yesterday I shown my Rust implementation of that distributed file system to my collaborator Chibin Zhang. He provided me with some useful suggestions after reviewing the code. After all it was seen as a toy example, I may update it in the future(有生之年).

Today I tried to do  a summary to smart pointer in Rust.

1. `Box<T>`

   It can allocate memory in heap for the data. In detailed implementation is use an keyword called `box`, which invokes `exchange_malloc()` and `box_free()` to complete the task of memory management.

2. `Rc` 

   It means reference counting, mainly to handle issues about shared ownership. There exist another similar design `Weak`(usually to solve recursive reference, which does increase reference counting), which does not have ownership but only borrow. Note that in condition of multi thread, you should consider `Arc` to guarantee the atomicity of operation.

3. `RefCell<T>`

   For an object instead of simple variable, we can use `Cell<T>` and `RefCell<T>` to realized the mutability of inner member. Note that the wrapped type in `Cell` must implement `Copy` trait, it has two method `set()` and `get()`. As to `RefCell`, it is more general, which also have two equivalent methods `borrow_mut()` and `borrow()` .

The rest of today, I planed to learn more about OS and finish part of rCore lab.

I read the graduation design of Runji Wang and learnt the starting point of `rCore`, it is so excited.

# Day7

Today I decided to have a rest and stop here for a while for I have TOFEL courses (they are expensive and meaningless.) . And I planed to update a note about RISC-V privileged architecture within coming three days.

# Day8

Today I went through the daily schedule of others. I found their rate of  process is ahead than mine. I have to work harder and harder to catch up with them.

# Day9

Today I almost finished lab 0, lab 1. In fact, it is not so hard. 

And I also post a GitHub issue, which indicated `asm!` had been deprecated.

But it does not matter, we can just use `llvm_asm!` instead.

# Day10

Today I learned more about RISC-V  assembly and its in-line form in Rust.  

[riscv-asm-manual](https://github.com/riscv/riscv-asm-manual/blob/master/riscv-asm.md)

And I tried first lab step by step.

And my yesterday's GitHub issue is about rCore-handbook, the rCore-tutorial is just up-to-data. That's fine.

A lot of changes exist between those versions of tutorial.

In source code assembly, `SAVE` and `LOAD` are two useful macros, their definitions begin with `.macro` and end with `.endm`. Here is an example:


```assembly
.macro LOAD reg, offset
  ld  \reg, \offset*8(sp)
.endm
```

Note that `sp` mean stack pointer register, it can be also written as `x2`

CSR about interrupt:

- `sepc`: exception program counter

- `scause`: whther hardware interrupt and its cause.

- `stval`: supervisor trap value register.

- `stvec`, `sstatus`, `sie`, `sip`, `scratch`

Related instructions:

- `ecall`: firmware relative call

- `sret`: kernel -> user

- `mret`: machine-level -> kernel-level

- `ebreak`: break at breakpoint

In rCore-Tutorial, those registers are binding with related structure, i.e. `Context`, we can refer to this easily.

And I use keyword `extern` frequently in os related work, so I need to clarify its exact meanings, 
you can find it in this link [extern](https://doc.rust-lang.org/std/keyword.extern.html)

# Day11-Day15

These days I have a lot of things to do so the process is much slower than before. And I also consult about the policy to enter Shen Zhen, but the teacher of administration told me that it may be troublesome for safety concerns. So Maybe I have to work online in later section.

# Day16

Today we have meeting about this activities, the teacher made an introduction to PengChengLab, it is wonderful. And most of questions about labs have been published. I started to write report for that.

# Day17

I finished report for lab 1.

# Day18

I finished most of lab 2. Report is on the way. 

# Day19

I finished most of lab 3. Report is on the way.

# Day20

I fixed the previous network problem of my desktop, and today I planed to finish lab 4. 

# Day21

I finished the report of lab 2 and lab 3. related material can be found in this repo.

