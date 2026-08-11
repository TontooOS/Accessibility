use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()));
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    let target_dir = manifest_dir.join("target").join(&profile);

    let lib_prefix = if cfg!(target_os = "linux") { "lib" } else { "" };
    let lib_ext = if cfg!(target_os = "linux") { "so" } else if cfg!(target_os = "macos") { "dylib" } else { "dll" };

    let so_name = format!("{}accessibility.{}", lib_prefix, lib_ext);
    let library_name = "accessibility.library";

    let so_path = target_dir.join(&so_name);
    let library_path = target_dir.join(library_name);

    let _ = fs::remove_file(&library_path);

    if so_path.exists() {
        match fs::copy(&so_path, &library_path) {
            Ok(_) => println!("cargo:warning=Created {}", library_name),
            Err(e) => println!("cargo:warning=Could not create {}: {}", library_name, e),
        }
    } else {
        println!("cargo:warning=SO file not found at {:?}", so_path);
    }

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/ffi.rs");
}
