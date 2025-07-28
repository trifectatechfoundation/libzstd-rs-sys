fn main() {
    #[cfg(target_arch = "x86_64")]
    cc::Build::new()
        .file("lib/decompress/huf_decompress_amd64.S")
        .compile("huf_decompress_amd64");
}
