use protobuf::{
    Message,
    plugin::{CodeGeneratorRequest, CodeGeneratorResponse},
};
use protoc_gen_py_pkg::process_request;
use std::io::{BufReader, Read, Write};

fn main() {
    env_logger::init();
    log::info!("Starting the application...");

    // Parse request into a struct
    let request = match parse_request() {
        Ok(req) => req,
        Err(e) => {
            log::error!("Failed to parse request: {}", e);
            std::process::exit(1);
        }
    };

    // Process the request and generate files
    let response = process_request(request);

    // Write response
    write_response(response);
}

fn parse_request() -> Result<CodeGeneratorRequest, protobuf::Error> {
    let mut request = CodeGeneratorRequest::new();
    request.merge_from_bytes(
        BufReader::new(std::io::stdin())
            .bytes()
            .filter_map(Result::ok)
            .collect::<Vec<u8>>()
            .as_slice(),
    )?;
    Ok(request)
}

fn write_response(response: CodeGeneratorResponse) {
    let output = response.write_to_bytes().unwrap();
    std::io::stdout().write_all(&output).unwrap()
}
