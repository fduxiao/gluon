use std::process::Command;
use std::env;

fn main() {
    if env::var("GIT_HASH").is_err() {
        let output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .unwrap();
        let git_hash = String::from_utf8(output.stdout).unwrap();
        println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    }
    // This is the closest thing to making sure we rebuild this every time a new commit is made
    println!("cargo:rerun-if-changed=../.git/COMMIT_EDITMSG");
}
