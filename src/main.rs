use protobuf::{
    descriptor::FileDescriptorProto, plugin::{code_generator_response::File, CodeGeneratorRequest, CodeGeneratorResponse}, Message
};
use protoc_gen_py_pkg::protos::py_package;
use std::{collections::HashMap, io::{BufReader, Read, Write}};

const CODE_GENERATOR_RESPONSE_FEATURE_PROTO3_OPTIONAL: u64 = 1;

fn main() {
    env_logger::init();
    log::info!("Starting the application...");
    
    // Parse request into a struct
    let request = parse_request();
    
    // Process the request and generate files
    let response = process_request(request);
    
    // Write response
    write_response(response);
}

fn parse_request() -> CodeGeneratorRequest {
    let mut request = CodeGeneratorRequest::new();
    request.merge_from_bytes(
        BufReader::new(std::io::stdin())
            .bytes()
            .filter_map(Result::ok)
            .collect::<Vec<u8>>()
            .as_slice(),
    ).unwrap();
    request
}

fn process_request(request: CodeGeneratorRequest) -> CodeGeneratorResponse {
    let mut response = CodeGeneratorResponse::new();
    response.set_supported_features(CODE_GENERATOR_RESPONSE_FEATURE_PROTO3_OPTIONAL);
    
    // Extract options from proto files
    let proto_options = extract_proto_options(&request);
    
    // Generate files based on options
    let files = generate_files(proto_options);
    
    // Add files to response
    for file in files {
        response.file.push(file);
    }
    
    response
}

fn extract_proto_options(request: &CodeGeneratorRequest) -> Vec<(&FileDescriptorProto, Option<py_package::PyPackageOptions>)> {
    request
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
        .collect()
}

fn generate_files(opts: Vec<(&FileDescriptorProto, Option<py_package::PyPackageOptions>)>) -> Vec<File> {
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

    output_files.into_values().collect()
}

fn write_response(response: CodeGeneratorResponse) {
    let output = response.write_to_bytes().unwrap();
    std::io::stdout().write_all(&output).unwrap()
}
