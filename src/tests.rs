use crate::with_alloca;

#[test]
fn test_create() {
    let x = with_alloca(4096, |_| {
        assert!(true);
        42
    });
    assert_eq!(x, 42);
}

#[test]
fn test_write() {
    let x = with_alloca(4096, |memory| {
        memory[0] = 42;
        memory[1] = 3;
        memory[3072] = 4;
        assert_eq!(memory[0], 42);
        assert_eq!(memory[1], 3);
        assert_eq!(memory[3072], 4);
        memory[0] + memory[1] + memory[3072]
    });
    assert_eq!(x, 42 + 3 + 4);
}
