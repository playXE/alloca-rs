use alloca::with_alloca;

fn main() {
    with_alloca(128, |_mem| {
        panic!("Hello!");
    });
}
