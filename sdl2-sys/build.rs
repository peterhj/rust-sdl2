use std::env;
use std::path::{PathBuf};

fn main() {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let host_triple = env::var("HOST").unwrap();
  let mut artifacts_path = PathBuf::from(&manifest_dir);
  artifacts_path.push(&format!("artifacts.{}", host_triple));
  println!("cargo:rustc-link-search=native={}", artifacts_path.to_str().unwrap());
}

/*#[cfg(feature="pkg-config")]
extern crate pkg_config;

fn main() {
    if !build_pkgconfig() {
        if cfg!(feature="use_mac_framework") {
            println!("cargo:rustc-flags=-l framework=SDL2");
        } else {
            println!("cargo:rustc-flags=-l SDL2");
        }
    }
}

#[cfg(not(feature="pkg-config"))]
fn build_pkgconfig() -> bool {
    false
}

#[cfg(feature="pkg-config")]
fn build_pkgconfig() -> bool {
    if pkg_config::find_library("sdl2").is_err() {
        panic!("Could not find SDL2 via pkgconfig");
    }
    true
}*/
