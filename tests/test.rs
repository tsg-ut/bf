use bf::compile;
use std::fs;

#[test]
fn test_unmatched_open() {
    let code = fs::read_to_string("examples/unmatched-open.bf").unwrap();
    assert!(compile(&code).is_err());
}

#[test]
fn test_unmatched_close() {
    let code = fs::read_to_string("examples/unmatched-close.bf").unwrap();
    assert!(compile(&code).is_err());
}
