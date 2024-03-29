fn main() {
    println!("cargo:rerun-if-changed=src/c_file.c");
    cc::Build::new().file("src/c_file.c").compile("hello_world");
}
