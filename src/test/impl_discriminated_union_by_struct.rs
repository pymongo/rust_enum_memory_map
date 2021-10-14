/*!
由于 Rust union 限制每个 variant 都要满足 Copy 和 non-Drop 所以只好自行实现定制的 enum
*/

use std::mem::size_of;

const STRING_SIZE: usize = size_of::<String>();
const PADDING_SIZE: usize = size_of::<usize>() - size_of::<Discriminant>();

// #[repr(u8)]
enum Discriminant {
    StackAlloc,
    HeapAlloc
}

// #[repr(packed)]
struct SmartAllocStr {
    discriminant: Discriminant,
    _padding: [u8; PADDING_SIZE],
    /// one of StackAllocStr(if str len <= 23) or String(if str len > 24)
    data: [u8; STRING_SIZE]
}

struct StackAllocStr {
    len: u8,
    data: [u8; STRING_SIZE - size_of::<u8>()]
}

impl Drop for SmartAllocStr {
    fn drop(&mut self) {
        // if self.discriminant == 1 {

        // }
    }
}

#[test]
fn feature() {
    dbg!(std::mem::size_of::<SmartAllocStr>());
}

// impl SmartStr {
//     fn new(s: &str) -> Self {
//         if s.len() >= std::mem::size_of::<String>() {
//             let mut self_ = Self {
//                 discriminant: 0,
//                 _padding: [0; 7],
//                 data: [0; STRING_SIZE],
//             };
//             let string = s.to_string();

//             std::mem::forget(string);
//             self_
//         } else {
//             todo!()
//         }
//     }
// }
