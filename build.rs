use std::path::Path;
use std::process::Command;

fn main() {
    let lib_dir = Path::new("libnl");

    println!("cargo:rerun-if-changed=libnl/Makefile");
    println!("cargo:rerun-if-changed=libnl/libnl.c");

    let status = Command::new("make")
        .current_dir(lib_dir)
        .status()
        .expect("Failed to run make");

    if !status.success() {
        panic!("make failed in libstacks/");
    }
}
