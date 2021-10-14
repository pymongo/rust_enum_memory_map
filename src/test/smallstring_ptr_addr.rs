use smartstring::{Compact, SmartString};

#[test]
fn smallstring_alloc_on_stack() {
    let smart = SmartString::<Compact>::from("hello world");
    let smart_meta = &smart as *const _;
    let smart_data = &smart.as_bytes()[0] as *const _;
    dbg!(smart_meta);
    dbg!(smart_data);
}

#[test]
fn smallstring_alloc_on_heap() {
    let smart = SmartString::<Compact>::from("a".repeat(100));
    let smart_meta = &smart as *const _;
    let smart_data = &smart.as_bytes()[0] as *const _;
    dbg!(smart_meta);
    dbg!(smart_data);
}
