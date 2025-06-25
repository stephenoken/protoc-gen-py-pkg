use protobuf::{
    descriptor::FileDescriptorProto, plugin::{code_generator_response::File, CodeGeneratorRequest, CodeGeneratorResponse}, Message
};
use protoc_gen_py_pkg::protos::py_package;
use std::{collections::HashMap, io::{BufReader, Read, Write}};

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
    let mut output_files: HashMap<String, File> = HashMap::new();
    
    opts.iter()
        .flat_map(|(file_descriptor, opts)| {
            // protoc_gen_py_pkg::generate_py_init_files(file_descriptor, opts)
            let configs = protoc_gen_py_pkg::generate_py_init_configs(
                file_descriptor,
                opts,
                protoc_gen_py_pkg::load_python_import_file(),
            );
            protoc_gen_py_pkg::generate_py_init_files(configs)
        })
        .for_each(|file| {
            if let Some(file_name) = file.name.as_ref(){
                log::info!("Generated file: {}", file_name);
                output_files.insert(file_name.clone(), file);
            } else {
                log::warn!("Generated file with no name, skipping.");
            }
        });

        output_files.iter()
        .for_each(|(_, file)| {
            response.file.push(file.clone());
        });

    let output = response.write_to_bytes().unwrap();
    std::io::stdout().write_all(&output).unwrap()
}
