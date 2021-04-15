use std::{env, path::PathBuf};

fn main() {
  println!("cargo:rerun-if-changed=vendor/wuffs/release/c/wuffs-v0.3.c");

  let mut cfg = cc::Build::new();
  let out = PathBuf::from(env::var("OUT_DIR").unwrap());

  cfg.flag_if_supported("/arch:AVX");

  cfg
    .opt_level(3)
    .define("WUFFS_IMPLEMENTATION", "1")
    .include("vendor/wuffs/release/c")
    .file("vendor/wuffs/release/c/wuffs-v0.3.c")
    .out_dir(out.clone())
    .compile("libwuffs.a");

  let bindings = bindgen::Builder::default()
    .header("src/lib.h")
    .allowlist_var(".*wuffs.*")
    .allowlist_type(".*wuffs.*")
    .allowlist_function(".*wuffs.*")
    .allowlist_var(".*WUFFS.*")
    .allowlist_type(".*WUFFS.*")
    .allowlist_function(".*WUFFS.*")
    .allowlist_recursively(true)
    // .allowlist_function("")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Failed to generate bindings");

  bindings
    .write_to_file(out.join("bindings.rs"))
    .expect("Failed to write bindings");
}
