fn main() {
    println!("cargo:rustc-link-search=/lib/arm-linux-gnueabihf");
    println!("cargo:rustc-link-lib=lirc_client");
}
