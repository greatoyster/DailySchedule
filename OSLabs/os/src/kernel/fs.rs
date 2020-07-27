//! 文件相关的内核功能

use super::*;
use crate::fs::ROOT_INODE;
use alloc::string::String;
use alloc::{vec, vec::Vec};
use bitflags::_core::ptr::slice_from_raw_parts_mut;
use bitflags::_core::slice::from_raw_parts;
use core::slice::from_raw_parts_mut;
use xmas_elf::sections::CompressionType::ProcessorSpecific;
use core::slice;
use core::str;

/// 从指定的文件中读取字符
///
/// 如果缓冲区暂无数据，返回 0；出现错误返回 -1
pub(super) fn sys_read(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    // 从进程中获取 inode
    let process = PROCESSOR.lock().current_thread().process.clone();
    if let Some(inode) = process.inner().descriptors.get(fd) {
        // 从系统调用传入的参数生成缓冲区
        let buffer = unsafe { from_raw_parts_mut(buffer, size) };
        // 尝试读取
        if let Ok(ret) = inode.read_at(0, buffer) {
            let ret = ret as isize;
            if ret > 0 {
                return SyscallResult::Proceed(ret);
            }
            if ret == 0 {
                return SyscallResult::Park(ret);
            }
        }
    }
    SyscallResult::Proceed(-1)
}

/// 将字符写入指定的文件
pub(super) fn sys_write(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    // 从进程中获取 inode
    let process = PROCESSOR.lock().current_thread().process.clone();
    if let Some(inode) = process.inner().descriptors.get(fd) {
        // 从系统调用传入的参数生成缓冲区
        let buffer = unsafe { from_raw_parts_mut(buffer, size) };
        // 尝试写入
        if let Ok(ret) = inode.write_at(0, buffer) {
            let ret = ret as isize;
            if ret >= 0 {
                return SyscallResult::Proceed(ret);
            }
        }
    }
    SyscallResult::Proceed(-1)
}

pub(super) fn sys_open(buffer: *mut u8, size: usize) -> SyscallResult {
    let mut file_name = unsafe { slice::from_raw_parts(buffer, size) };
    let file_name = str::from_utf8(file_name).unwrap();
    let file = ROOT_INODE.find(&file_name).unwrap();
    // println!("file_name: {}",file_name);
    let process = PROCESSOR.lock().current_thread().process.clone();
    process.inner().descriptors.push(file);
    let p1 = (PROCESSOR.lock().current_thread().process.clone().inner().descriptors.len() -1) as isize;
    println!("p1 {}", p1);
    // panic!();
    SyscallResult::Proceed(p1 as isize)
}

