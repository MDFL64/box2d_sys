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
        // --- Logic for using a pre-built MinGW DLL ---
        println!("cargo:rustc-link-lib=dylib=box2d"); // Name of your DLL (without .dll prefix/suffix)

        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let prebuilt_path = manifest_dir.join("prebuilt_mingw_libs");

        if !prebuilt_path.exists() {
            panic!(
                "The 'use_prebuilt_mingw_dll' feature is enabled, but the directory for prebuilt libraries does not exist: {:?}.\n\
                 Please create this directory and place your MinGW-compiled box2d.dll and box2d.lib (or box2d.lib (or box2d.lib)) into it.",
                prebuilt_path
            );
        }
        if !prebuilt_path.join("box2d.dll").exists() || !prebuilt_path.join("box2d.lib").exists() {
            panic!(
                "The 'use_prebuilt_mingw_dll' feature is enabled, but box2d.dll and/or its import library (box2d.lib) were not found in {:?}.",
                prebuilt_path
            );
        }

        println!("cargo:rustc-link-search=native={}", prebuilt_path.display());

        // Bindgen needs the Box2D C header files.
        // Assuming they are vendored in `box2d/include/` within the crate.
        let box2d_vendored_include_path = manifest_dir.join("box2d").join("include");
        if !box2d_vendored_include_path.exists() {
            panic!(
                "Box2D include path for prebuilt DLL scenario does not exist: {:?}. \
                 Ensure Box2D headers are vendored in box2d/include/ within the crate.",
                box2d_vendored_include_path
            );
        }
        generate_bindings(&box2d_vendored_include_path);

        // Return early as we are not building Box2D with CMake in this path
        return;
    }

    let mut box2d_config = cmake::Config::new("box2d"); // Path to Box2D source vendored in the crate

    // Common CMake definitions
    box2d_config
        .define("BOX2D_BUILD_UNIT_TESTS", "OFF")
        .define("BOX2D_BUILD_TESTBED", "OFF")
        .define("BOX2D_BUILD_DOCS", "OFF")
        .define("BOX2D_SAMPLES", "OFF")
        .define("BOX2D_BENCHMARKS", "OFF")
        .define("BOX2D_PROFILE", "OFF")
        .define("BOX2D_VALIDATE", "ON") // Keep validation on for debugging
        .define("ENKITS_BUILD_EXAMPLES", "OFF");
    // CMAKE_INSTALL_LIBDIR etc. are for the `install` step by CMake,
    // the `cmake` crate makes build artifacts available relative to `dst` (e.g., dst/lib, dst/bin)

    // Profile specific CMake settings
    let profile = env::var("PROFILE").unwrap_or_default();
    if profile == "release" {
        box2d_config.define("CMAKE_BUILD_TYPE", "Release");
    } else {
        box2d_config.define("CMAKE_BUILD_TYPE", "Debug");
    }

    #[cfg(feature = "no_avx2")]
    {
        box2d_config.define("BOX2D_AVX2", "OFF");
    }
    // (Add other CMake defines like BOX2D_ENABLE_SIMD if needed, based on Box2D's CMakeLists.txt defaults)
    // box2d_config.define("BOX2D_ENABLE_SIMD", "ON");

    // --- Non-Windows (Linux, macOS, etc.): Build as a Static Library ---
    println!("cargo:rustc-link-lib=static=box2d");
    box2d_config.define("BUILD_SHARED_LIBS", "OFF");

    let dst = box2d_config.build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    generate_bindings(&dst.join("include"));
}

// Helper function for bindgen to avoid code duplication
fn generate_bindings(box2d_include_path: &Path) {
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
        .clang_arg(format!("-I{}", box2d_include_path.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("__mingw_ldbl_type_t") // Keep if any chance of MinGW toolchain for bindgen
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
