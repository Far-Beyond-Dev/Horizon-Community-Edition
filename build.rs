use std::env;
use std::path::PathBuf;

fn main() {
    // Directory where the shared library is located
    let lib_dir = PathBuf::from("pebble-vault/go");

    // Add the directory to the search path for dynamic libraries
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    // Link the shared library
    println!("cargo:rustc-link-lib=dylib=pebble-vault");

    // Set the LD_LIBRARY_PATH environment variable for runtime
    let ld_library_path = env::var("LD_LIBRARY_PATH").unwrap_or_default();
    let new_ld_library_path = format!("{}:{}", lib_dir.display(), ld_library_path);
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", new_ld_library_path);
}