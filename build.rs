fn main() {
    println!("cargo:rerun-if-changed=src/sort.c");
    cc::Build::new().file("src/sort.c").compile("libcsort.a");
    println!("cargo:rerun-if-changed=src/sort.cpp");
    cc::Build::new()
        .file("src/sort.cpp")
        .cpp(true)
        .compile("libcppsort.a");
}
