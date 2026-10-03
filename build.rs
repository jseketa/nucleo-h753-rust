// pre-build steps are executed here.
// cargo hands the build script information through environment variables
// OUT_DIR is the path of a private folder where the script puts generated files.
// OUT_DIR is ALWAYS set by cargo and unwrap can't fail when run by cargo.

use std::env; // equivalent to #include <stdlib.h>
use std::path::PathBuf; // rust's path type, no more glueing strings together
use std::fs; // equivalent to #include <stdio.h>

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    // print an out_dir to the linker search path
    println!("cargo:rustc-link-search={}", out_dir.display()); // display() needed for paths
    let memory_x_destination = out_dir.join("memory.x");
    fs::copy("memory.x", memory_x_destination).unwrap();
    println!("cargo:rerun-if-changed=memory.x");
}
