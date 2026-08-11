use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let target_dir = PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string()));
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let deps_dir = target_dir.join(&profile).join("deps");

    let lib_prefix = if cfg!(target_os = "linux") { "lib" } else { "" };
    let lib_ext = if cfg!(target_os = "linux") { "so" } else if cfg!(target_os = "macos") { "dylib" } else { "dll" };

    let so_name = format!("{}tontoo_accessibility.{}", lib_prefix, lib_ext);
    let library_name = "accessibility.library";

    let so_path = deps_dir.join(&so_name);
    let library_path = deps_dir.join(library_name);

    let _ = fs::remove_file(&library_path);

    if so_path.exists() {
        fs::copy(&so_path, &library_name).unwrap_or_else(|e| {
            eprintln!("Warning: could not create {}: {}", library_name, e);
            0
        });
        println!("cargo:warning=Created {}", library_name);
    }

    println!("cargo:rerun-if-changed=src/lib.rs");
}
