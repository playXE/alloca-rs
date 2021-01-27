fn main() {
    cc::Build::new()
        .file("alloca_.c")
        .opt_level(3)
        .compile("calloca");
}
