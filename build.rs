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


// Get environment variable from string
fn get_env_var(var_name: &str) -> Option<String> {
    env::vars().filter_map(|t| {
        let (key, value) = t;
        if key == var_name {
            Some(value)
        } else {
            None
        }
    }).next()
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

    let static_libraries = feature!("static_libraries");
    let shared_libraries = match static_libraries {
        "ON" => "OFF",
        "OFF" => "ON",
        _ => "ON"
    };
    let library_type = match static_libraries {
        "ON" => "static",
        "OFF" => "dylib",
        _ => "static"
    };

    let mut dst_dir = "".to_owned();
    let mut lib_loc = Some("".to_owned());
    let mut inc_dir = Some("".to_owned());
    if cfg!(feature = "build_libraries") {
        let dst = Config::new("vendor")
            .define("CMAKE_INSTALL_LIBDIR", "lib")
            .define("BUILD_STATIC_LIBS", static_libraries)
            .define("BUILD_SHARED_LIBS", shared_libraries)
            .define("BUILD_TESTING", "OFF")
            .define("EXAMPLES_INSTALL", "OFF")
            .define("BUILD_ARKODE", feature!("arkode"))
            .define("BUILD_CVODE", feature!("cvode"))
            .define("BUILD_CVODES", feature!("cvodes"))
            .define("BUILD_IDA", feature!("ida"))
            .define("BUILD_IDAS", feature!("idas"))
            .define("BUILD_KINSOL", feature!("kinsol"))
            .define("OPENMP_ENABLE", feature!("nvecopenmp"))
            .define("PTHREAD_ENABLE", feature!("nvecpthreads"))
            .build();
        dst_dir = format!("{}", dst.display());
        lib_loc = Some(format!("{}/lib", dst_dir));
        inc_dir = Some(format!("{}/include", dst_dir));
    } else {
        lib_loc = get_env_var("SUNDIALS_LIBRARY_DIR"); 
        inc_dir = get_env_var("SUNDIALS_INCLUDE_DIR");
    }

    // Second, we let Cargo know about the library files

    match lib_loc {
        Some(loc) => println!("cargo:rustc-link-search=native={}", loc), 
        None => (),
    }
    println!("cargo:rustc-link-lib={}=sundials_nvecserial", library_type);
    println!("cargo:rustc-link-lib={}=sundials_sunlinsolband", library_type);
    println!("cargo:rustc-link-lib={}=sundials_sunlinsoldense", library_type);
    println!("cargo:rustc-link-lib={}=sundials_sunlinsolpcg", library_type);
    println!("cargo:rustc-link-lib={}=sundials_sunlinsolspbcgs", library_type);
    println!("cargo:rustc-link-lib={}=sundials_sunlinsolspfgmr", library_type);
    println!("cargo:rustc-link-lib={}=sundials_sunlinsolspgmr", library_type);
    println!("cargo:rustc-link-lib={}=sundials_sunlinsolsptfqmr", library_type);
    println!("cargo:rustc-link-lib={}=sundials_sunmatrixband", library_type);
    println!("cargo:rustc-link-lib={}=sundials_sunmatrixdense", library_type);
    println!("cargo:rustc-link-lib={}=sundials_sunmatrixsparse", library_type);
    println!("cargo:rustc-link-lib={}=sundials_sunnonlinsolfixedpoint", library_type);
    println!("cargo:rustc-link-lib={}=sundials_sunnonlinsolnewton", library_type);

    macro_rules! link {
        ($($s:tt),*) => {
            $(if cfg!(feature = $s) {
                println!("cargo:rustc-link-lib={}=sundials_{}", library_type, $s);
            })*
        }
    }
    link! {"arkode", "cvode", "cvodes", "cvodes", "ida", "idas", "kinsol", "nvecopenmp", "nvecpthreads"}

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
        .clang_arg(match inc_dir {
            Some(dir) => format!("-I{}", dir),
            None => "".to_owned(),
        })
        .clang_args(&[
            define!("arkode", ARKODE),
            define!("cvode", CVODE),
            define!("cvodes", CVODES),
            define!("ida", IDA),
            define!("idas", IDAS),
            define!("kinsol", KINSOL),
            define!("nvecopenmp", OPENMP),
            define!("nvecpthreads", PTHREADS),
        ])
        .parse_callbacks(Box::new(ParseSignedConstants))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // And that's all.
}
