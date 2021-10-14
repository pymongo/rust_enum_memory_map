/**
我还记得 Rust 编程之道还是实战课专门有章节提过这个问题
*/
#[test]
fn t1() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Bool {
        F,
        T,
    }   
    // 如果超出范围，则会 transmute 成第一个 variant
    let b: Bool = unsafe { std::mem::transmute(4_u8) };
    dbg!(b);
}

#[test]
fn transmute_u8_to_bool() {
    let b: bool = unsafe { std::mem::transmute(4_u8) };
    assert!(!b);
}
