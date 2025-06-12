use protobuf::{
    Message,
    plugin::{CodeGeneratorRequest, CodeGeneratorResponse},
};
use std::io::{BufReader, Read, Write};
pub mod protos;
use protos::py_package;

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

    let py_package_files = request
        .proto_file
        .iter()
        .map(|file| {
            let opts = py_package::exts::py_package_opts.get(&file.options);
            if let Some(o) = opts {
                log::info!("Found py_package options in file: {}", file.name());
                log::info!("Options: {:?}", o);
            };
            file
        })
        .collect::<Vec<_>>();

    // py_package_files.iter().for_each(|file| {
    //     log::info!("Found py_package file: {}", file.name());
    // });

    // request.proto_file.iter().for_each(|file| {
    //     log::info!("Found proto file: {}", file.name());
    //     log::info!("Options: {:?}", file.options.special_fields());
    //     py_package::file_descriptor().extensions().for_each(|ext| {
    //         if let Some(value) = file
    //             .options
    //             .special_fields()
    //             .unknown_fields()
    //             .iter()
    //             .find(|v| v.0 == ext.number() as u32)
    //         {
    //             log::info!("Extension {}: {:?}", ext.name(), value);
    //         } else {
    //             log::info!("Extension {} not set", ext.name());
    //         }
    //     });
    //     file.message_type.iter().for_each(|message| {
    //         log::info!("Message: {}", message.name());
    //     });
    // });
    // request.proto_file.iter().for_each(|file| {
    //     let file_name = file.name.as_deref().unwrap_or("unknown.proto");
    //     log::info!("Processing file: {}", file_name);

    //     let mut new_file = protobuf::plugin::code_generator_response::File::new();
    //     new_file.set_name(file_name.to_string());
    //     new_file.set_content(format!("// Generated code for: {}\n", file_name));
    //     response.file.push(new_file);
    // });
    let output = response.write_to_bytes().unwrap();
    std::io::stdout().write_all(&output).unwrap()
}
