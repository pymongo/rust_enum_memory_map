/*!
由于 Rust union 限制每个 variant 都要满足 Copy 和 non-Drop 所以只好自行实现定制的 enum
*/

use std::mem::size_of;

const STRING_SIZE: usize = size_of::<String>();
const PADDING_SIZE: usize = size_of::<usize>() - size_of::<Discriminant>();
const DATA_SIZE: usize = STRING_SIZE + PADDING_SIZE;

// #[repr(u8)]
#[derive(PartialEq)]
enum Discriminant {
    StackAlloc,
    HeapAlloc,
}

// #[repr(packed)]
struct SmartAllocStr {
    discriminant: Discriminant,
    /// one of StackAllocStr(if str len <= 31) or String(if str len >= 32)
    data: [u8; DATA_SIZE],
}

#[repr(packed)]
struct StackAllocStr {
    /// len range 0..=30
    len: u8,
    data: [u8; DATA_SIZE - size_of::<u8>()],
}

impl SmartAllocStr {
    fn new(s: &str) -> Self {
        if s.len() >= DATA_SIZE {
            let mut self_ = Self {
                discriminant: Discriminant::HeapAlloc,
                data: Default::default(),
            };
            let string = s.to_string();
            let ptr: *mut String = self_.data[..STRING_SIZE].as_mut_ptr().cast();
            unsafe { ptr.write_unaligned(string) };
            // std::mem::forget(string);
            self_
        } else {
            let mut self_ = Self {
                discriminant: Discriminant::StackAlloc,
                data: Default::default(),
            };
            let stack_str = StackAllocStr {
                len: 0,
                data: Default::default(),
            };
            let ptr: *mut StackAllocStr = self_.data[..STRING_SIZE].as_mut_ptr().cast();
            unsafe { ptr.write_unaligned(stack_str) };
            self_
            // Self {
            //     discriminant: Discriminant::StackAlloc,
            //     data: Default::default()
            // }
        }
    }
}

impl Drop for SmartAllocStr {
    fn drop(&mut self) {
        if self.discriminant == Discriminant::HeapAlloc {
            let s: *mut String = self.data[..STRING_SIZE].as_mut_ptr().cast();
            let b: String = unsafe { std::ptr::read_unaligned(s) };
            drop(b);
        }
    }
}

// cargo b && valgrind --tool=memcheck ./target/debug/smart_alloc_str
fn main() {
    let s = SmartAllocStr::new("hello");
}
