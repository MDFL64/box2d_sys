#![allow(unused_mut)]

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // Common rerun-if-changed directives
    println!("cargo:rerun-if-changed=box2d/"); // Vendored Box2D source
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=prebuilt_mingw_libs/"); // For the prebuilt DLL feature

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // Check for our feature flag first
    if target_os == "windows" && cfg!(feature = "use_prebuilt_mingw_dll") {
        panic!("functionality removed");
    }

    let crate_dir = env::current_dir().unwrap();
    let fake_sys_include = format!("-I{}/fake_sys_headers",crate_dir.to_str().unwrap());

    let mut box2d_config = cmake::Config::new("box2d"); // Path to Box2D source vendored in the crate

    let mut box2d_config = box2d_config
        .define("BOX2D_BUILD_UNIT_TESTS", "OFF")
        .define("BOX2D_UNIT_TESTS", "OFF")
        .define("BOX2D_BUILD_TESTBED", "OFF")
        .define("BOX2D_BUILD_DOCS", "OFF")
        .define("BOX2D_USER_SETTINGS", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BOX2D_SAMPLES", "OFF")
        .define("BOX2D_BENCHMARKS", "OFF")
        .define("BOX2D_DOCS", "OFF")
        .define("BOX2D_PROFILE", "OFF")
        .define("BOX2D_VALIDATE", "ON")
        .define("ENKITS_BUILD_EXAMPLES", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("CMAKE_INSTALL_BINDIR", "bin")
        .define("CMAKE_INSTALL_INCLUDEDIR", "include")
        .define("CMAKE_BUILD_TYPE", "Release")
    // extra hacks for wasm builds
        .define("CMAKE_C_COMPILER_WORKS", "1")
        .define("CMAKE_CXX_COMPILER_WORKS", "1")
        .cflag(&fake_sys_include);
        

    #[cfg(feature = "no_avx2")]
    {
        // for compatibility we can do this, itll make it slower tho so its not default
        box2d_config = box2d_config.define("BOX2D_AVX2", "OFF");
    }
    // (Add other CMake defines like BOX2D_ENABLE_SIMD if needed, based on Box2D's CMakeLists.txt defaults)
    // box2d_config.define("BOX2D_ENABLE_SIMD", "ON");

    // --- Non-Windows (Linux, macOS, etc.): Build as a Static Library ---
    println!("cargo:rustc-link-lib=static=box2d");

    let dst = box2d_config.build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    generate_bindings(&dst.join("include"), &fake_sys_include);
}

// Helper function for bindgen to avoid code duplication
fn generate_bindings(box2d_include_path: &Path, fake_sys_include: &str) {
    // Changed to &Path for flexibility
    if !box2d_include_path.exists() {
        panic!(
            "Box2D include path for bindgen does not exist: {:?}",
            box2d_include_path
        );
    }
    let wrapper_h_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("wrapper.h");
    if !wrapper_h_path.exists() {
        panic!("wrapper.h not found at {:?}", wrapper_h_path);
    }

    let bindings = bindgen::Builder::default()
        .header(wrapper_h_path.to_str().unwrap()) // Use path to wrapper.h
        //.clang_arg(format!("-I{}", box2d_include_path.display()))
        .clang_arg(fake_sys_include)
        .clang_arg("-fvisibility=default")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        /*.blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("__mingw_ldbl_type_t") // Keep if any chance of MinGW toolchain for bindgen
        //.emit_clang_ast() */
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
