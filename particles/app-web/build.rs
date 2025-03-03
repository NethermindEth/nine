use std::env;
use std::process::Command;
use walkdir::WalkDir;

fn main() {
    let ui_dir = "ui";

    // Traverse all files in the "ui" directory, but skip those under "ui/dist".
    for entry in WalkDir::new("ui").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            // Skip files that are within the "ui/dist" folder.
            let path = entry.path();
            if path.starts_with("ui/dist") || path.starts_with("ui/target") {
                continue;
            }
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    let is_release = env::var("PROFILE").unwrap() == "release";

    let mut cmd = Command::new("trunk");
    cmd.arg("build");

    if is_release {
        cmd.arg("--release");
    }

    let status = cmd
        .current_dir(ui_dir)
        .status()
        .expect("Failed to execute 'trunk build'");

    if !status.success() {
        panic!("'trunk build' failed with status: {}", status);
    }
}
