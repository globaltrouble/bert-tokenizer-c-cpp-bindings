extern crate cbindgen;

use std::env;
use std::path::Path;

fn main() {
    let pkg_name = env::var("CARGO_PKG_NAME").expect("no pkg name");
    let header_path = Path::new("target").join("include").join(pkg_name + ".h");
    println!(
        "cargo:warning=Creating header '{}'",
        header_path.to_str().expect("no header path")
    );

    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("no manifest");

    let mut config: cbindgen::Config = Default::default();
    config.cpp_compat = true;
    config.include_guard = Some(String::from("RUST_TOKENIZERS_H"));
    config.language = cbindgen::Language::C;

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file(header_path);
}
