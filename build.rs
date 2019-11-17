extern crate rustc_version;
use rustc_version::{version, Version};

fn main() {
    let version = version().unwrap();

    // Assert we haven't travelled back in time
    assert!(version.major >= 1);

    // Check for a minimum version for a few features
    if version >= Version::from((1, 31, 0)) {
        println!("cargo:rustc-cfg=allow_clippy");
    }
    if version >= Version::from((1, 36, 0)) {
        println!("cargo:rustc-cfg=maybe_uninit");
    }
    if version >= Version::from((1, 40, 0)) {
        println!("cargo:rustc-cfg=doctests");
    }
}
