/*!
由于 Rust union 限制每个 variant 都要满足 Copy 和 non-Drop 所以只好自行实现定制的 enum
*/

use std::mem::size_of;

const STRING_SIZE: usize = size_of::<String>();
const PADDING_SIZE: usize = size_of::<usize>() - size_of::<Discriminant>();
const DATA_SIZE: usize = STRING_SIZE + PADDING_SIZE;

// #[repr(u8)]
/// 既然 x86_64 寻址的时候指针中末尾 3 bit 永远为 0 (用不上)，可以把 discriminant 存在里面
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

/// warning: borrow of packed field is unsafe
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
            let ptr = self_.data[..STRING_SIZE].as_mut_ptr().cast::<String>();
            unsafe { ptr.write_unaligned(string) };
            // std::mem::forget(string);
            self_
        } else {
            let mut self_ = Self {
                discriminant: Discriminant::StackAlloc,
                data: Default::default(),
            };
            let mut stack_str = StackAllocStr {
                len: s.len() as _,
                data: Default::default(),
            };
            stack_str.data[..s.len()].copy_from_slice(s.as_bytes());
            let ptr = self_.data[..STRING_SIZE]
                .as_mut_ptr()
                .cast::<StackAllocStr>();
            // read/write unaligned 的终极解释:
            // 普通版 ptr::read/write 会按寄存器大小为最小单位去读内存
            // 但我 StackAllocStr 大小是 31 没必要读 32 个 byte
            // unaligned pointer 指的就是 repr(packed) 这种结构体，要用 unaligned 去读
            // 第二种 unaligned 的用法就是把两个 u8 当成一个 u16 去读，但会警告 clippy::cast_ptr_alignment
            // 或者从 [u8; 8] 的 [1..5] 读出一个 u32 也是没有按内存对齐去走性能会有所损失
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

impl AsRef<str> for SmartAllocStr {
    fn as_ref(&self) -> &str {
        match self.discriminant {
            Discriminant::StackAlloc => {
                let s = self.data.as_ptr().cast::<StackAllocStr>();
                unsafe {
                    let s = &*s;
                    let slice = std::slice::from_raw_parts(s.data.as_ptr(), usize::from(s.len));
                    std::str::from_utf8_unchecked(slice)
                }
            }
            Discriminant::HeapAlloc => {
                let s = self.data.as_ptr().cast::<std::mem::ManuallyDrop<String>>();
                let tmp = unsafe { s.read_unaligned() };
                unsafe { &*(tmp.as_ref() as *const str) }
            }
        }
    }
}

impl std::fmt::Display for SmartAllocStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = self.as_ref();
        std::fmt::Display::fmt(s, f)
    }
}

impl std::fmt::Debug for SmartAllocStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = self.as_ref();
        std::fmt::Debug::fmt(s, f)
    }
}

// TODO impl to &str for my kind

// cargo b && valgrind --tool=memcheck ./target/debug/smart_alloc_str
fn main() {
    dbg!(SmartAllocStr::new("hello"));
    dbg!(SmartAllocStr::new(&"a".repeat(60)));
}
