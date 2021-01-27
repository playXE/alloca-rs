use crate::with_alloca;
use core::mem::MaybeUninit;
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
        memory[0] = MaybeUninit::new(42);
        memory[1] = MaybeUninit::new(3);
        memory[3072] = MaybeUninit::new(4);
        unsafe {
            assert_eq!(memory[0].assume_init(), 42);
            assert_eq!(memory[1].assume_init(), 3);
            assert_eq!(memory[3072].assume_init(), 4);
            memory[0].assume_init() + memory[1].assume_init() + memory[3072].assume_init()
        }
    });
    assert_eq!(x, 42 + 3 + 4);
}
