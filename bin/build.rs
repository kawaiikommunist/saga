use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // 1. Tell Cargo to rerun this script if any file in your WASI library changes
    // Adjust the relative path to point to your WASI library crate directory
    println!("cargo:rerun-if-changed=../wasm/src");
    println!("cargo:rerun-if-changed=../wasm/Cargo.toml");

    // 2. Determine the profile (Debug vs Release) to match the host build
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let mut cargo_args = vec!["build", "--target", "wasm32-wasip2"];

    if profile == "release" {
        cargo_args.push("--release");
    }

    // 3. Run the cargo build command for the WASI component
    let isolated_target_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("wasi_target");

    let status = Command::new("cargo")
        .args(&cargo_args)
        .env("CARGO_TARGET_DIR", &isolated_target_dir) // Force isolated target
        .current_dir("../wasm") // Path to your WASI crate
        .status()
        .expect("Failed to execute cargo build for WASI component");

    if !status.success() {
        panic!("WASI component compilation failed.");
    }

    // --- UPDATE THE OUTPUT PATH ---
    // The WASM binary will now be located inside our isolated target directory
    let wasm_profile_dir = if profile == "release" {
        "release"
    } else {
        "debug"
    };
    let wasm_path = isolated_target_dir
        .join("wasm32-wasip2")
        .join(wasm_profile_dir)
        .join("wasm.wasm"); // Replace with your actual WASI crate name

    println!(
        "cargo:rustc-env=WASI_COMPONENT_PATH={}",
        wasm_path.display()
    );

    // // 4. (Optional) Pass the WASM file path to your main application via an env var
    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // // Cargo places the compiled wasm file in the workspace's target dir
    // // Adjust "wasi_components.wasm" to match your actual library crate name
    // let wasm_profile_dir = if profile == "release" {
    //     "release"
    // } else {
    //     "debug"
    // };
    // let wasm_path = PathBuf::from(env::current_dir().unwrap())
    //     .join("../target/wasm32-wasip1")
    //     .join(wasm_profile_dir)
    //     .join("wasi_components.wasm");

    // // Expose this path to your main.rs at compile time
    // println!(
    //     "cargo:rustc-env=WASI_COMPONENT_PATH={}",
    //     wasm_path.display()
    // );
}
