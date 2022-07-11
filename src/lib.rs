pub fn strip_0x(s: &str) -> &str {
    if &s[0..2] == "0x" {
        &s[2..]
    } else {
        s
    }
}

pub fn prepend_0x(s: &str) -> String {
    if &s[0..2] == "0x" {
        String::from(s)
    } else {
        format!("0x{}", s)
    }
}

/// RUST_LOG=debug cargo test --all-features --package prefix-manager --lib -- test_prefix --exact --show-output
#[test]
fn test_prefix() {
    assert_eq!(strip_0x("0xabc"), "abc");
    assert_eq!(prepend_0x("abc"), "0xabc");
}
