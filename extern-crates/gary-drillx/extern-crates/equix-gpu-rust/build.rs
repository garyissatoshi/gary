#[cfg(feature = "solve")]
fn main() {
    use cuda_builder::CudaBuilder;
    use std::env;
    use std::path;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=kernels");

    let out_path = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    CudaBuilder::new("kernels")
        .copy_to(out_path.join("kernels.ptx"))
        .build()
        .unwrap();
}

#[cfg(not(feature = "solve"))]
fn main() {}
