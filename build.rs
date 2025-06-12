use std::{fmt::Error, fs::create_dir_all, path::Path};

fn main() -> Result<(), Error> {
    let proto_dir = ".";
    let proto_file = Path::new(proto_dir).join("py_package.proto");
    let dest_path = Path::new("./src/protos");
    create_dir_all(dest_path).unwrap();

    println!("cargo:rerun-if-changed={}", proto_dir);
    println!("cargo:rerun-if-changed=build.rs");

    protobuf_codegen::Codegen::new()
        .protoc()
        .out_dir(dest_path)
        .includes(&[proto_dir])
        .input(proto_file.display().to_string())
        .run_from_script();
    println!("cargo:rerun-if-changed=src/protos");

    Ok(())
}
