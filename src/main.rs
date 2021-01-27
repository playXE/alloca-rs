use alloca::with_alloca;

fn main() {
    std::thread::Builder::new()
        .stack_size(32 * 1024)
        .spawn(|| {
            with_alloca(512 * 1024, |memory| {
                println!("{}", memory.len());
                println!("WIll write:");
                memory[512 * 1024 - 1] = 0;
            })
        })
        .unwrap()
        .join()
        .unwrap();
}
