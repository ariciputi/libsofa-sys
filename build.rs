use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use bindgen;
use cc;

fn main() {
    println!("cargo:rustc-link-lib=static=sofa_c");
    println!("cargo:rerun-if-changed=./vendor/src");

    let library_path = Path::new("./vendor/src");

    let mut src = fs::read_dir(library_path)
        .expect("Cannot access SOFA source directory.")
        .map(|res| res.map(|entry| entry.path()))
        .filter(|path| path.as_ref().unwrap().extension() == Some(OsStr::new("c")))
        .filter(|path| path.as_ref().unwrap().file_stem() != Some(OsStr::new("t_sofa_c")))
        .collect::<Result<Vec<_>, _>>()
        .expect("Cannot collect SOFA src files.");

    src.sort();

    let mut build = cc::Build::new();
    build
        .flag("-c")
        .flag("-pedantic")
        .flag("-Wall")
        .flag("-W")
        .flag("-O")
        .include(library_path)
        .files(src.iter())
        .compile("libsofa_c.a");

    let bindings = bindgen::Builder::default()
        .header("./vendor/src/sofa.h")
        .header("./vendor/src/sofam.h")
        .rustfmt_bindings(true)
        .whitelist_function("iau.*")
        .generate()
        .expect("Unable to generate bindings.");

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("Cannot get OUT_DIR env variable."));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings.");
}
