# Lab4-6报告

I checked the git repo of rCore-Tutorial, but found the rCore lab seem finished completely. Basic framework is added, but lack of more detailed descriptions and requirement. 

`File Organization`

There are a lot of content added in lab 4, Mainly in four submodule: `kernel`, `process`, `driver`, and `fs`

```shell
fs/
├── config.rs
├── inode_ext.rs
├── mod.rs
├── stdin.rs
└── stdout.rs
```

```
drivers/
├── block
│   ├── mod.rs
│   └── virtio_blk.rs
├── bus
│   ├── mod.rs
│   └── virtio_mmio.rs
├── device_tree.rs
├── driver.rs
└── mod.rs
```

```
kernel/
├── condvar.rs
├── fs.rs
├── mod.rs
├── process.rs
└── syscall.rs
```

```
process/
├── config.rs
├── kernel_stack.rs
├── lock.rs
├── mod.rs
├── process.rs
├── processor.rs
└── thread.rs
```

`File Descriptions`

The main file related to lab 4:

1. `process`

	- `config.rs`: contains STACK_SIZE and KERNEL_STACK_SIZE(both 512 KB)
	- `kernel_stack.rs`: which is just an array, support push context.
	- `lock.rs`: the `struct Lock` can be seen a wrapper of `Mutux`
	- `processor.rs` maintain the structure of processor, including the threads and relative scheduler.

`Practice`

1. Q1: which switch threads the processor will invoke `prepare_next_thread()` method to push context to user stack, related page table will be prepared at that time. The change in page table does not matter, as part of context, it is saved.
2. Q2: this part I referred to the official answer, but I still have questions about the last case.
3. Q3&Q4 is implementing…
4. 

