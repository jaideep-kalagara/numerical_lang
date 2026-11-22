use std::process::Command;

fn main() {
    let status = Command::new("make")
        .current_dir("nl")
        .status()
        .expect("Failed to compile libnl");

    if !status.success() {
        panic!("Failed to build libnl");
    }

    println!("cargo:rerun-if-changed=nl/libnl.c");
}
