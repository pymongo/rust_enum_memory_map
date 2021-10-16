# enum 的内存布局/内存对齐/大小

## 假设有这样的面试题

在 x86_64-unknown-linux-gnu 的 TARGET 下请回答以下 enum 的大小 (std::mem::size_of)

### u64 and *const c_char

```rust
enum UserID {
    Number(u64),
    Text(*const c_char),
}
```

指针和 u64 大小都是 u64, tag 是 u8 

所以 enum 大小是: 1(tag) + 7(padding) + 8(union u64)

### u64 * u8 with repr(packed)

```rust
#[repr(packed)]
struct UserID {
    number: u64,
    text: u8
}
```

`#[repr(packed)]` 等同于 C 语言的 `__attribute__((packed))` 可以阻止编译器对结构体进行内存对齐优化(将结构体的大小填充为 寄存器大小(64 位 CPU 是 8 byte) 的整数倍)

试想下如果没有 packed 两个在内存中相邻的结构体一个长度是 9 另一个长度是 7

此时两个 struct 有 1 byte 是 overlapping(重叠的) 而 CPU 寄存器最小读取单位是 8 byte

所以 CPU 要连续读两个 8 byte 才能解析出结构体 1 而且还导致两个结构体的数据在同一个 8 byte 内存单元中带来诸多问题

注意 Rust 的 enum 没法用 repr(packed)

借用 packed 结构体的字段时会抛出警告:

> warning: borrow of packed field is unsafe

所以 struct 大小是: 8 + 1

### u64 and String

```rust
enum UserID {
    Number(u64),
    Text(String),
}
```

String/Vec 的大小是 8(ptr)+8(len)+8(cap) 所以 enum 大小是 1(tag) + 7(padding) + 24 = 32

## 为什么 enum 是和类型

```rust
enum SumType {
    A(bool),
    B(bool),
    C(bool)
}

struct ProductType {
    a: bool,
    b: bool,
    c: bool
}
```

`enum SumType` 和 `struct ProductType` 都有 3 个 bool 类型"字段"(准确来说是 variant)，bool 只有两种状态

`enum SumType` 一共有 2+2+2 种状态，`struct ProductType` 一共有 `2*2*2` 种状态

所以说 enum 是和类型，状态的数量等于各个 variant 状态数量之和，而 struct 是积类型

## discriminated/tagged union

Rust 这种和类型的 enum 在其它语言中也叫 tagged union 或者 discriminated union

## too large on stack

如果有个变量大小超过 200 且分配在栈上，clippy 有警告

clippy_lints/src/utils/conf.rs:

> (too_large_for_stack: u64 = 200)

但如果大小很大的变量仅仅在函数内使用没有在函数间传递倒也不用在意这个 lint

再看看 clippy.toml 的这个配置项:

> (pass_by_value_size_limit: u64 = 256)

很容易想到学 C 语言提过体积特别大的结构体在函数间传递时要 malloc 分配到堆上然后传递指针

clippy 中也有一个类似的设置选项:

> (enum_variant_size_threshold: u64 = 200)



