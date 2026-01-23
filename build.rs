#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    println!("cargo:rustc-link-search=native=/opt/homebrew/opt/openssl/lib");
    println!("cargo:include=/opt/homebrew/opt/openssl/include");
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=crypto");

    println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/libevent/2.1.12_1/lib");
    println!("cargo:include=/opt/homebrew/Cellar/libevent/2.1.12_1/include");
    println!("cargo:rustc-link-lib=static=event");
}
