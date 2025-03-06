// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rustc-link-lib=example");

    println!("cargo:rustc-link-search={path}", path = "./target/debug");
    println!("cargo:rustc-link-search={path}", path = "./target/release");

    println!("cargo::rustc-link-arg=-Wl,-rpath={path}", path = ".");
    println!("cargo::rustc-link-arg=-Wl,-rpath={path}", path = "./target/debug");
    println!("cargo::rustc-link-arg=-Wl,-rpath={path}", path = "./target/release");
}