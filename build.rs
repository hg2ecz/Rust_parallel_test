fn main() {
    cc::Build::new()
        .file("src/conv.c")
        .flag("-O3") // -O3 or -Ofast (unsafe parallel)
        .flag("-march=native")
        .flag("-funroll-all-loops")
        .compile("libconv.a");
}
