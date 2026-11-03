fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_unix = std::env::var_os("CARGO_CFG_UNIX").is_some();
    if target_arch == "x86_64" && target_unix {
        cc::Build::new()
            .file("lib/decompress/huf_decompress_amd64.S")
            .compile("huf_decompress_amd64");
    }
}
