use rustc_version;

fn main() {
    let version = rustc_version::version().unwrap();
    println!("cargo:rustc-env=RUSTC_VERSION={}", version);

    capnpc::CompilerCommand::new()
        .output_path("src")
        .file("iomod.capnp")
        .run()
        .unwrap();
}
