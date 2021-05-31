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

#[test]
fn test_cat() {
    let code = fs::read_to_string("examples/cat.bf").unwrap();
    let program = compile(&code).unwrap();
    assert_eq!(program.run(b"meow"), Some(b"meow".to_vec()));
}

#[test]
fn test_hello() {
    // Note: hello.bf needs negative memory addresses
    let code = fs::read_to_string("examples/hello.bf").unwrap();
    let program = compile(&code).unwrap();
    assert_eq!(program.run(b""), Some(b"Hello, World!".to_vec()));
}
