use std::io::{BufReader, Read, Write};

use protobuf::{plugin::{CodeGeneratorRequest, CodeGeneratorResponse}, Message};

const CODE_GENERATOR_RESPONSE_FEATURE_PROTO3_OPTIONAL: u64 = 1;

fn main() {
    env_logger::init();
    log::info!("Starting the application...");
    let mut request = CodeGeneratorRequest::new();
    request.merge_from_bytes(
        BufReader::new(std::io::stdin()).bytes()
            .filter_map(Result::ok)
            .collect::<Vec<u8>>()
            .as_slice(),
    ).unwrap();

    let mut response = CodeGeneratorResponse::new();
    response.set_supported_features(CODE_GENERATOR_RESPONSE_FEATURE_PROTO3_OPTIONAL);
    request.proto_file.iter().for_each(|file| {
        log::info!("Found proto file: {}", file.name());
        log::info!("Options: {:?}", file.options.special_fields());
        file.message_type.iter().for_each(|message| {
            log::info!("Message: {}", message.name());
        });
    });
    request.proto_file.iter().for_each(|file| {
        let file_name = file.name.as_deref().unwrap_or("unknown.proto");
        log::info!("Processing file: {}", file_name);


        let mut new_file = protobuf::plugin::code_generator_response::File::new();
        new_file.set_name(file_name.to_string());
        new_file.set_content(format!("// Generated code for: {}\n", file_name));
        response.file.push(new_file);
    });
    let output = response.write_to_bytes().unwrap();
    std::io::stdout().write_all(&output).unwrap()
}
