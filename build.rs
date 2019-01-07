use cmake::Config;
use std::env;
use std::path::PathBuf;

// SUNDIALS has a few non-negative constants that need to be parsed as an i32.
// This is an attempt at doing so generally.
#[derive(Debug)]
struct ParseSignedConstants;

impl bindgen::callbacks::ParseCallbacks for ParseSignedConstants {
    fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
        let prefix: String = name.chars().take_while(|c| *c != '_').collect();
        match prefix.as_ref() {
            "CV" | "IDA" | "KIN" | "SUN" => Some(bindgen::callbacks::IntKind::Int),
            _ => None,
        }
    }
}

fn main() {
    // First, we build the SUNDIALS library, with requested modules with CMake

    macro_rules! feature {
        ($s:tt) => {
            if cfg!(feature = $s) {
                "ON"
            } else {
                "OFF"
            }
        };
    }

    let dst = Config::new("vendor")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("BUILD_STATIC_LIBS", "OFF")
        .define("BUILD_SHARED_LIBS", "ON")
        .define("BUILD_TESTING", "OFF")
        .define("EXAMPLES_INSTALL", "OFF")
        .define("BUILD_ARKODE", feature!("arkode"))
        .define("BUILD_CVODE", feature!("cvode"))
        .define("BUILD_CVODES", feature!("cvodes"))
        .define("BUILD_IDA", feature!("ida"))
        .define("BUILD_IDAS", feature!("idas"))
        .define("BUILD_KINSOL", feature!("kinsol"))
        .define("OPENMP_ENABLE", feature!("openmp"))
        .build();

    // Second, we let Cargo know about the library files

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=sundials_nvecserial");
    println!("cargo:rustc-link-lib=dylib=sundials_sunlinsolband");
    println!("cargo:rustc-link-lib=dylib=sundials_sunlinsoldense");
    println!("cargo:rustc-link-lib=dylib=sundials_sunlinsolpcg");
    println!("cargo:rustc-link-lib=dylib=sundials_sunlinsolspbcgs");
    println!("cargo:rustc-link-lib=dylib=sundials_sunlinsolspfgmr");
    println!("cargo:rustc-link-lib=dylib=sundials_sunlinsolspgmr");
    println!("cargo:rustc-link-lib=dylib=sundials_sunlinsolsptfqmr");
    println!("cargo:rustc-link-lib=dylib=sundials_sunmatrixband");
    println!("cargo:rustc-link-lib=dylib=sundials_sunmatrixdense");
    println!("cargo:rustc-link-lib=dylib=sundials_sunmatrixsparse");
    println!("cargo:rustc-link-lib=dylib=sundials_sunnonlinsolfixedpoint");
    println!("cargo:rustc-link-lib=dylib=sundials_sunnonlinsolnewton");

    macro_rules! link {
        ($($s:tt),*) => {
            $(if cfg!(feature = $s) {
                println!("cargo:rustc-link-lib=dylib=sundials_{}", $s);
            })*
        }
    }
    link! {"arkode", "cvode", "cvodes", "cvodes", "ida", "idas", "kinsol"}
    if cfg!(feature = "openmp") {
        println!("cargo:rustc-link-lib=dylib=sundials_nvecopenmp");
    }

    // Third, we use bindgen to generate the Rust types

    macro_rules! define {
        ($a:tt, $b:tt) => {
            format!(
                "-DUSE_{}={}",
                stringify!($b),
                if cfg!(feature = $a) { 1 } else { 0 }
            )
        };
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}/include", dst.display()))
        .clang_args(&[
            define!("arkode", ARKODE),
            define!("cvode", CVODE),
            define!("cvodes", CVODES),
            define!("ida", IDA),
            define!("idas", IDAS),
            define!("kinsol", KINSOL),
            define!("openmp", OPENMP),
        ])
        .parse_callbacks(Box::new(ParseSignedConstants))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // And that's all.
}
