// extern crate bindgen;

// use std::env;
// use std::path::PathBuf;

// fn main() {
//     // Tell cargo to look for shared libraries in the specified directory
//     println!("cargo:rustc-link-search=/abseil/abseil-cpp/absl/algorithm");

//     // Tell cargo to tell rustc to link the system bzip2
//     // shared library.
//     // println!("cargo:rustc-link-lib=absl");

//     // Tell cargo to invalidate the built crate whenever the wrapper changes
//     println!("cargo:rerun-if-changed=wrapper.h");

//     // The bindgen::Builder is the main entry point
//     // to bindgen, and lets you build up options for
//     // the resulting bindings.
//     let bindings = bindgen::Builder::default()
//         // The input header we would like to generate
//         // bindings for.
//         .header("wrapper.h")
//         // Tell cargo to invalidate the built crate whenever any of the
//         // included header files changed.
//         .parse_callbacks(Box::new(bindgen::CargoCallbacks))
//         // Finish the builder and generate the bindings.
//         .generate()
//         // Unwrap the Result and panic on failure.
//         .expect("Unable to generate bindings");

//     // Write the bindings to the $OUT_DIR/bindings.rs file.
//     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//     bindings
//         .write_to_file(out_path.join("bindings.rs"))
//         .expect("Couldn't write bindings!");
// }

// fn main() -> miette::Result<()> {
//     let path = std::path::PathBuf::from("src"); // include path
//     let mut b = autocxx_build::Builder::new("src/main.rs", &[&path]).build()?;
//         // This assumes all your C++ bindings are in main.rs
//     b.flag_if_supported("-std=c++14")
//      .compile("autocxx-demo"); // arbitrary library name, pick anything
//     println!("cargo:rerun-if-changed=src/main.rs");
//     // Add instructions to link to any C++ libraries you need.
//     Ok(())
// }

extern crate autocxx_build;

fn main()-> miette::Result<()> {
    let path = std::path::PathBuf::from("/s2/s2geometry/src/s2"); // include path
    let path4 = std::path::PathBuf::from("/s2/s2geometry/src"); // include path
    let path5 = std::path::PathBuf::from("/abseil/abseil-cpp"); // include path
    let path1 = std::path::PathBuf::from("/abseil/abseil-cpp/absl/base"); // include path
    let path2 = std::path::PathBuf::from("/abseil/abseil-cpp/absl/algorithm"); // include path
    let path3 = std::path::PathBuf::from("/abseil/abseil-cpp/absl/cleanup"); // include path
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&path, &path2, &path3, &path1, &path4, &path5]).build()?;
        // .extra_clang_args(&["-std=c++17"])
        // .expect_build();
    b.flag_if_supported("-std=c++14")
        .compile("s2geometry-rust"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/main.rs");
    // Add instructions to link to any C++ libraries you need.
    Ok(())
}