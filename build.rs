use std::process::Command;

fn main() {
    let status = Command::new("cargo")
        .current_dir("nl")
        .args(["build", "--release"])
        .status()
        .expect("Failed to compile libnl");

    if !status.success() {
        panic!("Failed to build libnl");
    }

    println!("cargo:rerun-if-changed=nl/src");
    println!("cargo:rerun-if-changed=nl/Cargo.toml");
    println!("cargo:rerun-if-changed=nl/rust-toolchain.toml");
}
