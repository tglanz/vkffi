// build.rs

use std::env;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    /**
     * fuckckckc it was tough...
     * the first line is the -L argument to gcc
     * where the second line is -l
     */
    println!("cargo:rustc-link-search={}", r"C:\VulkanSDK\1.0.3.1\Bin"); // the "-L" flag
    println!("cargo:rustc-link-lib=vulkan-1"); // the "-l" flag
}
