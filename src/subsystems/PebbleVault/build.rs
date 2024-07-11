extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

     // Define the path to the go directory
     let go_dir = Path::new("./go");

     // Change the current directory to the go directory before running the go build command
     assert!(env::set_current_dir(&go_dir).is_ok(), "Failed to change directory to ./go");

    let mut go_build = Command::new("go");
    go_build
        .arg("build")
        .arg("-buildmode=c-archive")
        .arg("-o")
        .arg(out_path.join("libgo.a"))
        .arg("./main.go");

    go_build.status().expect("Go build failed");

    // Change back to the original directory if needed
    //assert!(env::set_current_dir("/PebbleVault").is_ok(), "Failed to change back to the original directory");

    let bindings = bindgen::Builder::default()
        .header(out_path.join("libgo.h").to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=go/lib.go");
    println!(
        "cargo:rustc-link-search=native={}",
        out_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static={}", "go");
}