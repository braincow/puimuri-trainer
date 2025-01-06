use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=frontend/");

    let is_release = env::var("PROFILE").unwrap() == "release";

    // Build the Vue/Vite frontend
    if is_release {
        // Install npm dependencies
        let npm_install_output = Command::new("sh")
            .arg("-c")
            .arg("npm install")
            .current_dir("./frontend")
            .output()
            .expect("Failed to execute npm install");

        if !npm_install_output.status.success() {
            panic!(
                "npm install failed:\n{}",
                String::from_utf8_lossy(&npm_install_output.stderr)
            );
        }

        println!("Running npm run build (release mode)...");
        let npm_build_output = Command::new("sh")
            .arg("-c")
            .arg("npm run build")
            .current_dir("./frontend")
            .output()
            .expect("Failed to execute npm run build");

        if !npm_build_output.status.success() {
            panic!(
                "npm run build failed:\n{}",
                String::from_utf8_lossy(&npm_build_output.stderr)
            );
        }
    }
}
