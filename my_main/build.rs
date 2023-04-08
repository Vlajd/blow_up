fn main() {
    println!("cargo:rustc-link-search=static=d:/Projekte/vs/rust/blow_up/target/debug");
    println!("cargo:rustc-link-lib=static=my_lib");
}
