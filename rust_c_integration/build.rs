fn main() {
    // cc::Build comes from the cc crate — make sure it's in Cargo.toml
    cc::Build::new()
        .cpp(true)
        .file("cpp/add.cpp")
        .include("cpp")
        .compile("add");

    println!("cargo:rerun-if-changed=cpp/add.cpp");
    println!("cargo:rerun-if-changed=cpp/add.h");
}
