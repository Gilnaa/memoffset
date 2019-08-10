extern crate rustc_version;
use rustc_version::{version, version_meta, Channel, Version};

fn main() {
    // Assert we haven't travelled back in time
    assert!(version().unwrap().major >= 1);

    // Check for a minimum version
    if version().unwrap() >= Version::parse("1.36.0").unwrap() {
        println!("cargo:rustc-cfg=memoffset_maybe_uninit");
    }

    // Check for nightly.
    if let Channel::Nightly = version_meta().unwrap().channel {
        println!("cargo:rustc-cfg=memoffset_nightly");
    }
}
