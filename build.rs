fn main() {
    let mut builder = cc::Build::new();
    builder.flag_if_supported("-fstack-clash-protection");
    (if option_env!("CC") == Some("clang") {
        builder.flag("-flto")
    } else {
        &mut builder
    })
    .file("alloca_.c")
    .opt_level(2)
    .compile("calloca");
}
