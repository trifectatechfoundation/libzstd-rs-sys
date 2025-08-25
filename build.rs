fn main() {
    // The actual target of the compilation. When cross-compiling this is different from the host
    // target. Build scripts are always built with the host target as the target.
    let target = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    if target == "x86_64" {
        cc::Build::new()
            .file("lib/decompress/huf_decompress_amd64.S")
            .compile("huf_decompress_amd64");
    }
}
