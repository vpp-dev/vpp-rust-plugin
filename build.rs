extern crate bindgen;

use std::env;
use std::path::PathBuf;
use bindgen::builder;
use std::process::Command;


fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    // let mut bindings = bindgen::Builder::default()
    let mut bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-I../vpp/build-root/install-vpp_debug-native/vpp/include")
        .no_convert_floats()
        .hide_type("max_align_t")
        .opaque_type("vnet_main_t")
        .opaque_type("vnet_hw_interface_t")
        .opaque_type("vnet_sw_interface_t")
        .hide_type("vnet_sw_interface_t")
        .generate()
        // Finish the builder and generate the bindings.
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

/*

The manipulations with types above are because bindgen does not handle this
type of C code:

struct blah_t;

typedef void *(blah_callback_t)(struct blah_t *param);

typedef struct {
  blah_callback_t *blah_callback;
} blah_t;

*/

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file_name = out_path.join("bindings.rs");
    bindings
        .write_to_file(out_file_name.clone())
        .expect("Couldn't write bindings!");

    Command::new("rustup").args(&["run", "nightly", "rustfmt", out_file_name.to_str().unwrap()]).status(); // .unwrap();
}


