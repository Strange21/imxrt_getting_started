use imxrt_rt::{Family, RuntimeBuilder};
// use std::path::PathBuf;
// use std::env;

fn main() {
    // let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    RuntimeBuilder::from_flexspi(Family::Imxrt1060, 16 * 1024 * 1024)
        .build()
        .unwrap();

        // println!("cargo:rustc-link-search={}", out.display());
}