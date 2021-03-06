# Lab2报告

`File Organization`

Compared with previous lab 1, lab 2 add two individual modules `memory` and `algorithm`.

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
   - `address.rs`: it contains the basic implementation of physical address concept in OS, maybe in lab 3, the struct `PhysicalAddress` bind a `usize`, which is a unit struct. it have method `page_offset()` to get offset in physical page. Here it also implemented a useful macro to cover all `usize` operations, just like operator override in CXX.
   - `heap.rs`: just a simple implementation for heap, given base address and range.
   - `range.rs`: it provides us with a convenient interface of iterator in those type related to address.
   - `frame/allocator.rs`: this part is about allocator
2. `algorithm`: its submodule `allocator` provides us with a bit map allocator and stacked allocator. 
   - `stacked_allocator.rs`: the stacked allocator is used for the allocation of physical pages. It use tuples to represent available spaces. so new with `(0,capacity)`, when allocating spaces, it pops the last tuple to see whether there is free space, if so, it will push a new tuple to update.

`Practice`

- Q1: we store those uninitialized global variables in bss segment, its full name is block started by symbol. The heap is actually uninitialized data and it is seen as a global variable in our os implementation.

- Q2: it is somehow like a chicken-egg problem, such code will not work.

- Q3.1: space O(n), time O(1).

- Q3.2: implementing, you can see the code

  ```rust
  //! Segment tree allocator
  use super::Allocator;
  use alloc::{vec, vec::Vec};
  
  struct SegmentTree {
      left: usize,
      right: usize,
  }
  
  pub struct SegmentTreeAllocator {
      capacity: usize,
      tree: Vec<SegmentTree>,
      data: Vec<usize>,
  }
  
  impl SegmentTreeAllocator {
      pub fn init(cap: usize) -> Self {
          let mut sta = SegmentTreeAllocator {
              capacity: cap,
              tree: Vec::<SegmentTree>::new(),
              data: Vec::<usize>::new(),
          };
          for _ in 0..cap * 4 + 5 as usize {
              sta.tree.push(SegmentTree { left: 0, right: 0 });
          }
          sta.data.push(0);
          for _ in 0..cap as usize {
              sta.data.push(0);
          }
          sta
      }
      pub fn build(&mut self, rt: usize, l: usize, r: usize) {
          self.tree[rt].left = l;
          self.tree[rt].right = r;
          if l == r {
              // update
              return;
          }
          let mid = (l + r) / 2;
          self.build(rt * 2, l, mid);
          self.build(rt * 2 + 1, mid + 1, r);
          // update
      }
  }
  
  
  impl Allocator for SegmentTreeAllocator {
      fn new(capacity: usize) -> Self {
          SegmentTreeAllocator::init(capacity)
      }
      fn alloc(&mut self) -> Option<usize> {
          let mut alloc_success = false;
          let mut start = 0;
          for i in 1..self.capacity + 1 as usize {
              if self.data[i] == 0 {
                  alloc_success = true;
                  start = i - 1;
                  self.data[i] = 1;
                  break;
              }
          }
          if alloc_success {
              Some(start)
          } else { None }
      }
  
      fn dealloc(&mut self, index: usize) {
          self.data[index + 1] = 0;
      }
  }
  
  ```

  And I have a question here, how to use segment tree to allocator memory efficiently? I think my implementation is somehow not so fast which is `O(n)`. Anyway, it works.
  
  Few days later, I found the correct implementation in the lab 4 frame work 
  
  ```rust
  use super::Allocator;
  use alloc::{vec, vec::Vec};
  use bit_field::BitArray;
  pub struct SegmentTreeAllocator {
      tree: Vec<u8>,
  }
  impl Allocator for SegmentTreeAllocator {
      fn new(capacity: usize) -> Self {
          assert!(capacity >= 8);
          let leaf_count = capacity.next_power_of_two();
          let mut tree = vec![0u8; 2 * leaf_count];
          for i in ((capacity + 7) / 8)..(leaf_count / 8) {
              tree[leaf_count / 8 + i] = 255u8;
          }
          for i in capacity..(capacity + 8) {
              tree.set_bit(leaf_count + i, true);
          }
          for i in (1..leaf_count).rev() {
              let v = tree.get_bit(i * 2) && tree.get_bit(i * 2 + 1);
              tree.set_bit(i, v);
          }
          Self { tree }
      }
      fn alloc(&mut self) -> Option<usize> {
          if self.tree.get_bit(1) {
              None
          } else {
              let mut node = 1;
              while node < self.tree.len() / 2 {
                  if !self.tree.get_bit(node * 2) {
                      node *= 2;
                  } else if !self.tree.get_bit(node * 2 + 1) {
                      node = node * 2 + 1;
                  } else {
                      panic!("tree is full or damaged");
                  }
              }
               assert!(!self.tree.get_bit(node), "tree is damaged");
               self.update_node(node, true);
               Some(node - self.tree.len() / 2)
          }
      }
      fn dealloc(&mut self, index: usize) {
          let node = index + self.tree.len() / 2;
          assert!(self.tree.get_bit(node));
          self.update_node(node, false);
      }
  }
  impl SegmentTreeAllocator {
      fn update_node(&mut self, mut index: usize, value: bool) {
          self.tree.set_bit(index, value);
          while index > 1 {
              index /= 2;
              let v = self.tree.get_bit(index * 2) && self.tree.get_bit(index * 2 + 1);
              self.tree.set_bit(index, v);
          }
      }
  }
  
  ```
  
  