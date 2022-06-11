use std::process::Command;
use std::env;
use std::path::{Path};

fn main() {
    let path = Path::new("./libjit");

    env::set_current_dir(path.to_str().expect("expect libjit to exist in cargo outdir")).expect("expected libjit to exist in out dir");

    Command::new("./bootstrap")
        .output().expect("Failed to run ./bootstrap installing libjit");
    Command::new("./configure")
        .output().expect("Failed to run ./configure while installing libjit");
    Command::new("make")
        .output().expect("Failed to run make while installing libjit");
    Command::new("make").args(&["install"])
        .output().expect("Failed to run make install while installing libjit");

    let library_path = path.join("jit").join(".libs");

    println!("cargo:rustc-link-search={}", library_path.to_str().expect("libjit './jit/.libs' directory was not created during installation for some reason"));
    println!("cargo:rustc-link-lib=jit.0");
    println!("cargo:rerun-if-changed=build.rs");
}
