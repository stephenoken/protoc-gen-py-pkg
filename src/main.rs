use protobuf::{
    Message,
    plugin::{CodeGeneratorRequest, CodeGeneratorResponse},
};
use std::{io::{BufReader, Read, Write}, iter, path::Iter};
// pub mod protos;
use protoc_gen_py_pkg::protos::py_package;

// use protoc_gen_py_pkg::protos::py_package::exts::py_package_opts;

const CODE_GENERATOR_RESPONSE_FEATURE_PROTO3_OPTIONAL: u64 = 1;


fn main() {
    env_logger::init();
    log::info!("Starting the application...");
    let mut request = CodeGeneratorRequest::new();
    request
        .merge_from_bytes(
            BufReader::new(std::io::stdin())
                .bytes()
                .filter_map(Result::ok)
                .collect::<Vec<u8>>()
                .as_slice(),
        )
        .unwrap();

    let mut response = CodeGeneratorResponse::new();
    response.set_supported_features(CODE_GENERATOR_RESPONSE_FEATURE_PROTO3_OPTIONAL);

    let opts: Vec<Option<py_package::PyPackageOptions>> = request
        .proto_file
        .iter()
        .map(|file| {
            let opts = py_package::exts::py_package_opts.get(&file.options);
            if let Some(opt) = &opts {
                log::info!("Found py_package options in file: {}", file.name());
                log::info!("Options: {:?}", opt);
            };
            opts
        })
        .collect();

    opts.iter().flat_map(protoc_gen_py_pkg::generate_py_init_files).for_each(|file| {
        log::info!("Generated file: {}", file.name());
        response.file.push(file);
    });
    
    let output = response.write_to_bytes().unwrap();
    std::io::stdout().write_all(&output).unwrap()
}
