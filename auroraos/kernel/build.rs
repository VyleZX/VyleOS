//! AuroraOS Kernel Build Script

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell Cargo to rerun if these files change
    println!("cargo:rerun-if-changed=build.rs");
    
    // Set up linker script for the kernel
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Copy linker script to output directory
    // This will be used by the custom target specification
    
    #[cfg(target_arch = "x86_64")]
    {
        println!("cargo:rustc-link-arg=-Tkernel_link.ld");
    }
}
