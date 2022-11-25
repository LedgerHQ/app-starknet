fn main() {
    println!("cargo:rerun-if-changed=script.ld");
    //println!("cargo:rustc-cfg=debug");
}
