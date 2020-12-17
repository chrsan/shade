use std::env;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    let mut build = cxx_build::bridges(&["src/core.rs", "src/skottie.rs", "src/svg.rs"]);
    build
        .define("SK_RELEASE", None)
        .include("skia")
        .file("cc/core.cc")
        .file("cc/skottie.cc")
        .file("cc/svg.cc")
        .flag("-std=c++17")
        .flag("-fno-exceptions")
        .flag("-fno-rtti");
    build.compile("shade");
    write_compile_flags(build.get_compiler().args()).unwrap();

    let lib_dir = Path::new(MANIFEST_DIR)
        .join("lib")
        .join(if cfg!(target_os = "macos") {
            "darwin"
        } else {
            "linux"
        })
        .join(&env::var("CARGO_CFG_TARGET_ARCH").unwrap());

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=skia");
    println!("cargo:rustc-link-lib=static=skshaper");
    println!("cargo:rustc-link-lib=static=skottie");
    println!("cargo:rustc-link-lib=static=sksg");
    println!("cargo:rustc-link-lib=static=svg");

    println!("cargo:rerun-if-changed=src/core.rs");
    println!("cargo:rerun-if-changed=cc/core.h");
    println!("cargo:rerun-if-changed=cc/core.cc");
    println!("cargo:rerun-if-changed=src/skottie.rs");
    println!("cargo:rerun-if-changed=cc/skottie.h");
    println!("cargo:rerun-if-changed=cc/skottie.cc");
    println!("cargo:rerun-if-changed=src/svg.rs");
    println!("cargo:rerun-if-changed=cc/svg.h");
    println!("cargo:rerun-if-changed=cc/svg.cc");
}

fn write_compile_flags(args: &[std::ffi::OsString]) -> Result<()> {
    let path = Path::new(MANIFEST_DIR).join("compile_flags.txt");
    let mut file = File::create(&path)?;
    for arg in args {
        writeln!(&mut file, "{}", arg.to_str().unwrap())?;
    }
    writeln!(&mut file, "-xc++")?;
    Ok(())
}
