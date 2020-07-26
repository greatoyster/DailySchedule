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
	
	- `thread.rs`: these code is similar to `process.rs`, thread has its own stack and process, and it use `ThreadInner` to indicate the state of  a thread. I have a question here about thread ID
	
	  ```rust
	  /// 线程 ID 使用 `isize`，可以用负数表示错误
	  pub type ThreadID = isize;
	  ```
	
	  Why can not we to use a `Result` or `Option` to encode Thread ID, then it can use `usize` which can represent more threads. though both of them is OK.   
	
2. `kernel`

   - `condvar.rs`: contains a struct `Condvar` which is a mutex contains atomic reference counting of threads. It has tow methods: `wait` (push current thread to deque and sleep) and `notify_one` (wake and pop the first thread in deque).
   - `process.rs`: it is a system call of exit, the detailed definition of system call is not here.
   - `syscall.rs`: it is about system call, if you want to realize more system call, you can add new case to the match arms in the `syscall_handler`. The framework have implemented the read, write, and exit three system calls. 

3. `fs` and `driver`: these two parts have fewer codes than other modules, the external device support and file system operation is implemented in these two modules.

`Lab 4 Practice`

1. Q1: which switch threads the processor will invoke `prepare_next_thread()` method to push context to user stack, related page table will be prepared at that time. The change in page table does not matter, as part of context, it is saved.

2. Q2: this part I referred to the official answer, but I still have questions about the last case.

3. Q3: to kill a thread we just need to do two steps, just add these two lines to the interrupt handler while receiving `ctrl+c`

   ```rust
   processor.kill_current_thread();
   processor.prepare_next_thread();
   ```

   ```rust
   fn supervisor_external(context: &mut Context) -> *mut Context {
       let mut c = console_getchar();
       println!("{}", c);
       if c <= 255 {
           if c == '\r' as usize {
               c = '\n' as usize;
           }
           STDIN.push(c as u8);
           if c == 3 {
              PROCESSOR.lock().kill_current_thread();
              PROCESSOR.lock().prepare_next_thread();
           }
       }
       context
   }
   ```

4. Q4, this part is a little bit confusing. just as the interface, you need to impl `processor.rs`

   ```rust
    pub fn fork_current_thread(&mut self, context: &Context) {
           let thread = self.current_thread().fork(context).unwrap();
           self.scheduler.add_thread(thread)
    }
   ```

   Then you need to implement `fork` in `threads`, just like its `new()`

   ```rust
   pub fn fork(&self, current_context: &Context) -> MemoryResult<Arc<Thread>> {
           println!("~~~FORK U~~~");
           let process = self.process.clone();
           let stack = process.alloc_page_range(STACK_SIZE, Flags::READABLE | Flags::WRITABLE)?;
           for i in 0..STACK_SIZE {
               *VirtualAddress(stack.start.0 + i).deref::<u8>() = *VirtualAddress(self.stack.start.0 + i).deref::<u8>()
           }
           let mut context = current_context.clone();
           context.set_sp(usize::from(stack.start) - usize::from(self.stack.start) + current_context.sp());
           let thread = Arc::new(Thread {
               id: unsafe {
                   THREAD_COUNTER += 1;
                   THREAD_COUNTER
               },
               stack,
               process,
               inner: Mutex::new(ThreadInner {
                   context: Some(context),
                   sleeping: false,
                   dead: false,
               }),
           });
           Ok(thread)
       }
   ```

   But I still confused about previous discussion about the semantics of `fork` and  `clone`

   

`Lab 6 pratice`

1. Q1: in aspect of thread, it is blocked, but in aspect of OS, it is not so, because OS can do others thing while waiting.

2. Q2: if you want to use `Vec`, you need to allocate free memory in runtime, then the heap must be involved, and to use more space than user stack, maybe you need to use virtual memory.  

3. Q3&Q4: to implement system `get_tid`(), take it easy, first you had better need to add these constant 

   ```rust
   pub const SYS_GET_TID: usize = 94;
   pub const SYS_CLONE: usize = 95;
   ```

   Then add an arm metion above in `os/syscall.rs`

   ```
   let result = match syscall_id {
           SYS_READ => sys_read(args[0], args[1] as *mut u8, args[2]),
           SYS_WRITE => sys_write(args[0], args[1] as *mut u8, args[2]),
           SYS_EXIT => sys_exit(args[0]),
           SYS_GET_TID => sys_get_tid(),
           SYS_CLONE => sys_clone(context),
           _ => {
               println!("unimplemented syscall: {}", syscall_id);
               SyscallResult::Kill
           }
       };
   ```

   

