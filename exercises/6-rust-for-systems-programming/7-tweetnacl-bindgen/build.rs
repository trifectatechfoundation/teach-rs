fn main() {
    println!("cargo:rerun-if-changed=tweetnacl.h");
    println!("cargo:rerun-if-changed=tweetnacl.c");

    cc::Build::new()
        .warnings(false)
        .extra_warnings(false)
        .file("tweetnacl.c")
        .compile("tweetnacl"); // outputs `libtweetnacl.a`
}
