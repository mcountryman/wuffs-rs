use std::{env, path::PathBuf};

fn main() {
  let mut cfg = cc::Build::new();
  let out = PathBuf::from(env::var("OUT_DIR").unwrap());

  cfg.flag_if_supported("/arch:AVX");

  cfg
    .include("vendor/wuffs/release/c")
    .file("vendor/wuffs/release/c/wuffs-v0.3.c")
    .out_dir(out.join("lib"))
    .compile("libwuffs.lib");

  let bindings = bindgen::Builder::default()
    .header("src/lib.h")
    // .allowlist_function("")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Failed to generate bindings");

  bindings
    .write_to_file(out.join("bindings.rs"))
    .expect("Failed to write bindings");
}
