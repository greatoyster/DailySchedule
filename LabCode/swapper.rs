//! 页面置换算法

use super::*;
use crate::memory::{frame::FrameTracker, *};
use alloc::collections::VecDeque;
use riscv::register::fcsr::Flag;

/// 管理一个线程所映射的页面的置换操作
pub trait Swapper {
    /// 新建带有一个分配数量上限的置换器
    fn new(quota: usize) -> Self;

    /// 是否已达到上限
    fn full(&self) -> bool;

    /// 取出一组映射
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)>;

    /// 添加一组映射（不会在以达到分配上限时调用）
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry);

    /// 只保留符合某种条件的条目（用于移除一段虚拟地址）
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool);
}

pub type SwapperImpl = ClockSwapper;

/// 页面置换算法基础实现：FIFO
pub struct FIFOSwapper {
    /// 记录映射和添加的顺序
    queue: VecDeque<(VirtualPageNumber, FrameTracker)>,
    /// 映射数量上限
    quota: usize,
}

impl Swapper for FIFOSwapper {
    fn new(quota: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            quota,
        }
    }
    fn full(&self) -> bool {
        self.queue.len() == self.quota
    }
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        self.queue.pop_front()
    }
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, _entry: *mut PageTableEntry) {
        self.queue.push_back((vpn, frame));
    }
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _)| predicate(vpn));
    }
}


pub struct ClockSwapper {
    queue: VecDeque<(VirtualPageNumber, FrameTracker, PageTableEntry)>,
    quota: usize,
}

impl Swapper for ClockSwapper {
    /// 新建带有一个分配数量上限的置换器
    fn new(quota: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            quota,
        }
    }

    /// 是否已达到上限
    fn full(&self) -> bool {
        self.queue.len() >= self.quota
    }

    /// 取出一组映射
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        let num_of_pages = self.queue.len();
        let mut clock_pointer: usize = 0;
        #[derive(PartialOrd, PartialEq, Eq)]
        pub enum SwapperState {
            Finding,
            Found,
        }
        let mut state = SwapperState::Finding;
        unsafe {
            while state == SwapperState::Finding {
                if clock_pointer == num_of_pages  {
                    let (vpn, ft, _) = self.queue.pop_front().unwrap();
                    return Some((vpn, ft));
                }
                let mut pte = self.queue[clock_pointer].2.clone();
                if !pte.is_accessed(){
                    let (vpn, ft, _) = self.queue.remove(clock_pointer).unwrap();
                    return Some((vpn, ft));
                }
                clock_pointer+=1;
            }
        }

        None
    }

    /// 添加一组映射（不会在以达到分配上限时调用）
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry) {
        unsafe {
            self.queue.push_back((vpn, frame, *entry));
        }
    }

    /// 只保留符合某种条件的条目（用于移除一段虚拟地址）
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _, _)| predicate(vpn));
    }
}
