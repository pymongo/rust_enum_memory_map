// clippy::trivially_copy_pass_by_ref
fn a(a: &[u8; 8 - 1]) {}

fn a_2(a: &[u8; 8]) {}

// clippy::large_types_passed_by_value
fn b(b: [u8; 256 + 1]) {}

fn b2(b: [u8; 256]) {}
