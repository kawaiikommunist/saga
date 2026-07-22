use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Rerun script if mod_source folder or Cargo.toml changes
    println!("cargo:rerun-if-changed=../mod_source");
    println!("cargo:rerun-if-changed=../Cargo.toml");

    // Locate workspace root relative to builder/
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir
        .parent()
        .expect("Failed to locate workspace root")
        .to_path_buf();

    let out_dir = workspace_root.join("mods");
    let mod_source_dir = workspace_root.join("mod_source");
    let target_dir = workspace_root.join("target/wasm32-unknown-unknown/release");

    // Ensure output directory exists
    fs::create_dir_all(&out_dir).expect("Failed to create mods directory");

    // Find all packages inside mod_source/
    let mut mod_packages = Vec::new();
    if mod_source_dir.exists() {
        if let Ok(entries) = fs::read_dir(&mod_source_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && path.join("Cargo.toml").exists() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        mod_packages.push(name.to_string());
                    }
                }
            }
        }
    }

    if mod_packages.is_empty() {
        println!("cargo:warning=No crates found in mod_source/");
        return;
    }

    // Compile each package to wasm32-unknown-unknown
    let mut cargo_cmd = Command::new("cargo");
    cargo_cmd
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release");

    for package in &mod_packages {
        cargo_cmd.arg("-p").arg(package);
    }

    let status = cargo_cmd
        .status()
        .expect("Failed to execute cargo build command");

    if !status.success() {
        panic!("Compilation of WASM mods failed.");
    }

    // Copy generated .wasm files to mods/
    for package in &mod_packages {
        // Cargo turns hyphens into underscores in output binaries
        let wasm_file_name = format!("{}.wasm", package.replace('-', "_"));
        let src_path = target_dir.join(&wasm_file_name);
        let dest_path = out_dir.join(&wasm_file_name);

        if src_path.exists() {
            fs::copy(&src_path, &dest_path)
                .unwrap_or_else(|e| panic!("Failed to copy {}: {}", wasm_file_name, e));
            println!("cargo:warning=Exported: {} -> mods/", wasm_file_name);
        } else {
            println!(
                "cargo:warning=Expected output file not found: {}",
                src_path.display()
            );
        }
    }
}
