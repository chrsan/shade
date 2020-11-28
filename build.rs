const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let dummy = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("dummy.cc");
    std::fs::write(
        &dummy,
        format!(
            "const char* SHADE_VERSION = \"{}\";",
            env!("CARGO_PKG_VERSION")
        ),
    )
    .unwrap();

    cc::Build::new()
        .cpp(true)
        .file(&dummy)
        .flag("-std=c++17")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .compile("shade");

    let lib_dir = std::path::Path::new(MANIFEST_DIR)
        .join("lib")
        .join(if cfg!(target_os = "macos") {
            "darwin"
        } else {
            "linux"
        })
        .join(&std::env::var("CARGO_CFG_TARGET_ARCH").unwrap());

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=skia");
    println!("cargo:rustc-link-lib=static=skshaper");
    println!("cargo:rustc-link-lib=static=skottie");
    println!("cargo:rustc-link-lib=static=sksg");
    println!("cargo:rustc-link-lib=static=svg");
    println!("cargo:rustc-link-lib=static=shade_capi");
}
