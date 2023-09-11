use std::env;
use std::path::PathBuf;

static SRCDIR: &str = "SAM/sam";
static _LIBS: [&str;5] = ["reciter.o", "sam.o", "render.o", "lib.o", "debug.o"]; 

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rerun-if-changed={}/*",SRCDIR);
    let mut builder = cc::Build::new();

    builder.files(
        _LIBS.map(|lib|{
            format!("{SRCDIR}/{}",lib.replace(".o",".c"))
        })
    ).flag("-static").compile("sam");

    println!("cargo:rustc-link-lib=sam");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("SAM/sam/lib.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
