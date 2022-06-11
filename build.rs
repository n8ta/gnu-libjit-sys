use std::process::Command;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let path = Path::new("./libjit");


    Command::new("cp").args(&["-r", path.to_str().expect("converting ./libjit to path"), &out_dir])
        .spawn().expect("copying lib jit to cargo outdir");

    let libjit_out_path = PathBuf::from(out_dir).join("libjit");

    env::set_current_dir(libjit_out_path.to_str().expect("expect libjit to exist in cargo outdir")).expect("expected libjit to exist in out dir");

    Command::new("./bootstrap")
        .spawn().expect("Failed to run ./bootstrap installing libjit");
    Command::new("./configure")
        .spawn().expect("Failed to run ./configure while installing libjit");
    Command::new("make")
        .spawn().expect("Failed to run make while installing libjit");
    Command::new("make").args(&["install"])
        .spawn().expect("Failed to run make install while installing libjit");

    let library_path = libjit_out_path.join("jit").join(".libs");

    println!("cargo:rustc-link-search={}", library_path.to_str().expect("libjit './jit/.libs' directory was not created during installation for some reason"));
    println!("cargo:rustc-link-lib=jit.0");
    println!("cargo:rerun-if-changed=build.rs");
}
