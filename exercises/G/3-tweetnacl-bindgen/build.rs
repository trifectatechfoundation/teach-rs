fn main() {
    cc::Build::new()
        .warnings(false)
        .extra_warnings(false)
        .file("tweetnacl.c")
        .compile("tweetnacl"); // outputs `libtweetnacl.a`
}
