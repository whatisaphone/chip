use std::{env, path::PathBuf};

fn main() {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from(root).join("upstream");
    println!("cargo:rustc-link-search={}", path.to_str().unwrap());
    println!("cargo:rustc-link-lib=bot_utils");
}
