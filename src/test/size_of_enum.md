# enum 的内存布局/内存对齐/大小

## 假设有这样的面试题

在 x86_64-unknown-linux-gnu 的 TARGET 下请回答以下 enum 的大小 (std::mem::size_of)

### u64 and String

```rust
enum UserID {
    Number(u64),
    Text(String),
}
```

u64 + String 答案是 32

String 是胖指针，大小是 usize * 2 所以是 16 byte

TODO

### u64 and *const c_char

```rust
enum UserID {
    Number(u64),
    Text(*const c_char),
}
```

u64 + *const c_char 答案是 16

指针和 u64 大小都是 u64 然后还有 8 byte 是 enum 的 tag ?

TODO
