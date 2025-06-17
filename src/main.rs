use protobuf::{
    Message,
    descriptor::FileDescriptorProto,
    plugin::{CodeGeneratorRequest, CodeGeneratorResponse},
};
use std::{
    io::{BufReader, Read, Write},
    iter,
    path::Iter,
};
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
    let opts: Vec<(&FileDescriptorProto, Option<py_package::PyPackageOptions>)> = request
        .proto_file
        .iter()
        .map(|file| {
            let opts = py_package::exts::py_package_opts.get(&file.options);
            if let Some(opt) = &opts {
                log::info!("Found py_package options in file: {}", file.name());
                log::info!("Options: {:?}", opt);
            };
            (file, opts)
        })
        .collect();

        /* 
        We won't use protoc to generate the files 
        instead we will return a dictionary of the init files 
        along with the top level message object that's going to be inserted.
        We will then check if the init file exists at the top level.
        If it doesn't exist we will create it and write out the top level 
        imports. However, if the file does exist we will read the contents 
        of the file line by line and then create a a set and then output the contents 
        to the new file.
         */
    let file = opts.iter()
        .flat_map(|(file_descriptor, opts)| {
            protoc_gen_py_pkg::generate_py_init_files(file_descriptor, opts)
        })
        .map(|file| {
            log::info!("Generated file: {}", file.name());
            let file_name = String::from(file.name());
            response.file.push(file);
            file_name
        });

    let output = response.write_to_bytes().unwrap();
    std::io::stdout().write_all(&output).unwrap()
}
