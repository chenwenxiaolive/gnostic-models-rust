use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let proto_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("proto");

    let proto_files = &[proto_root.join("openapiv2.proto")];

    let include_dirs = &[proto_root.clone()];

    prost_build::Config::new()
        .compile_protos(proto_files, include_dirs)?;

    for proto in proto_files {
        println!("cargo:rerun-if-changed={}", proto.display());
    }

    Ok(())
}
