### alloca-rs

Mostly safe no_std wrapper for `alloca` in Rust.

This crate uses Rust lifetime system to ensure that stack allocated memory will not be used after function return, but
it does not make any guarantee about memory that is turned into raw pointer and stored somewhere else.

### Example

```rust
fn main() {
    // allocate 128 bytes on the stack
    alloca::with_alloca(128, |memory| {
        // memory: &mut [MaybeUninit<u8>]
        assert_eq!(memory.len(), 128);
    });
}
```
