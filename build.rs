use std::{path::PathBuf, env};

fn main() {
  println!("cargo:rerun-if-changed=src");
  println!("cargo:rerun-if-changed=build.rs");
  let files = std::fs::read_dir("src/c_lib").unwrap();
  let res = files.map(|file| {file.unwrap().path()});
  cc::Build::new().cpp(true).files(res).compile("zero");

  let bindings = bindgen::Builder::default()
  .header("src/c_lib/app.cpp")
  .clang_args(&["-x", "c++"])
  .allowlist_type("Human")
  .allowlist_function("print_human")
  .allowlist_function("print_car")
  .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
      bindings
          .write_to_file(out_path.join("bindings.rs"))
          .expect("Couldn't write bindings!");
}