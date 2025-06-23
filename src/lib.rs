pub mod protos;
use std::{collections::HashMap, iter::Scan, ops::Range, path::{self, Path}};

use protobuf::{descriptor::FileDescriptorProto, plugin::code_generator_response::File};
use protos::py_package;

use crate::protos::py_package::PyPackageOptions;

pub struct InitPyConfig {
    py_imports: Vec<String>,
}

pub fn generate_py_init_configs(file_descriptor: &FileDescriptorProto, opts: &Option<py_package::PyPackageOptions>) -> HashMap<String, InitPyConfig> {
    let mut configs: HashMap<String, InitPyConfig> = HashMap::new();
    opts.as_ref().into_iter()
    .filter(|opt| opt.enable)
    .flat_map(|opt| build_init_file_paths(opt, file_descriptor))
    .for_each(|(opts, file_path)| {
        let path = Path::new(&file_path);
        let parent_dir = path.parent().unwrap_or_else(|| Path::new(""));
        let parent_dir_str = parent_dir.to_string_lossy().to_string();
        let init_py_config = configs.entry(parent_dir_str).or_insert_with(|| InitPyConfig {
            py_imports: Vec::new(),
        });
        let py_module_path = &file_descriptor.name().split("/").map(|part| {
            part.replace(".proto", "_pb2")
        }).collect::<Vec<_>>().join(".");
        let python_imports = file_descriptor.message_type.iter()
        .filter(|_| {
           opts.enable_top_level_imports && parent_dir.components().count() == 1 
        })
        .map(|message| {
            format!("from {} import {}", py_module_path, message.name())
        }).collect::<Vec<_>>();
        init_py_config.py_imports.extend(python_imports);
    });
    configs
}

fn build_init_file_paths<'a>(
    opts: &'a py_package::PyPackageOptions,
    file_descriptor: &FileDescriptorProto,
) -> Scan<Range<usize>, String, impl FnMut(&mut String, usize) -> Option<(&'a py_package::PyPackageOptions, String)>>
{
    let components: Vec<_> = file_descriptor.name().split('/').collect();
    // Use scan to accumulate path components while yielding each one.
    (0..components.len() - 1).scan(String::new(), move |path_so_far, index| {
        // let root_component = components.first();
        let current_component = components[index];
        let path_dir = if path_so_far.is_empty() {
            Path::new(&current_component).to_path_buf()
        } else {
            Path::new(&path_so_far).join(current_component)
        };
        // Update the path_so_far with the current component
        *path_so_far = path_dir.to_string_lossy().to_string();
        let file_name = path_dir.join("__init__.py").to_string_lossy().to_string();
        Some((opts, file_name))
    })
}

pub fn generate_py_init_files(
    file_descriptor: &FileDescriptorProto,
    opts: &Option<py_package::PyPackageOptions>,
) -> impl Iterator<Item = File> {
    // Creates an iterator with 0 or 1 items based on whether `opts` is `Some` or `None`.
    opts
        // Using as_ref to convert the Option into an Option<&PyPackageOptions>.
        // This allows us to avoid moving the value out of the Option.
        .as_ref()
        .into_iter()
        .filter(|opt| opt.enable)
        // Using flat map to iterate over the option and generate an iterator of `File` objects.
        .flat_map(|opt| create_init_files(opt, file_descriptor))
        
}

fn create_init_files(
    _: &py_package::PyPackageOptions,
    file_descriptor: &FileDescriptorProto,
) -> Scan<Range<usize>, String, impl FnMut(&mut String, usize) -> Option<(File)>>
{
    let components: Vec<_> = file_descriptor.name().split('/').collect();
    // Use scan to accumulate path components while yielding each one.
    (0..components.len() - 1).scan(String::new(), move |path_so_far, index| {
        // let root_component = components.first();
        let current_component = components[index];
        let path_dir = if path_so_far.is_empty() {
            Path::new(&current_component).to_path_buf()
        } else {
            Path::new(&path_so_far).join(current_component)
        };
        // Update the path_so_far with the current component
        *path_so_far = path_dir.to_string_lossy().to_string();
        let file_name = path_dir.join("__init__.py").to_string_lossy().to_string();
        let mut file = File::new();
        file.set_name(file_name);
        Some(file)
    })
}

#[cfg(test)]
mod tests {
    use protobuf::descriptor::FileDescriptorProto;

    use super::*;

    #[test]
    fn it_should_generate_py_init_configs() {
        let mut opts = py_package::PyPackageOptions::new();
        opts.enable = true;
        opts.enable_top_level_imports = true;
        let mut file_descriptor = FileDescriptorProto::new();
        file_descriptor.set_name(String::from("example/v1/foo.proto"));
        let mut foo = protobuf::descriptor::DescriptorProto::new();
        foo.set_name(String::from("Foo"));
        file_descriptor.message_type.push(foo);
        let binding = Some(opts);
        let result = generate_py_init_configs(&file_descriptor, &binding);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("example"));
        assert!(result.contains_key("example/v1"));
        if let Some(init_config) = result.get("example") {
            assert_eq!(init_config.py_imports.len(), 1);
            assert_eq!(init_config.py_imports[0], "from example.v1.foo_pb2 import Foo");
        }
        if let Some(init_config) = result.get("example/v1") {
            assert_eq!(init_config.py_imports.len(), 0);
        }
    }

    #[test]
    fn it_should_generate_py_init_files() {
        let mut opts = py_package::PyPackageOptions::new();
        opts.enable = true;
        let mut file_descriptor = FileDescriptorProto::new();
        file_descriptor.set_name(String::from("example/v1/foo_pb2.py"));
        // opts.name = String::from("example.v1");
        let binding = Some(opts);
        let result: Vec<_> = generate_py_init_files(&file_descriptor, &binding).collect();
        assert_eq!(result.len(), 2);
        if let Some(name) = &result[0].name {
            assert_eq!(name, "example/__init__.py");
        }
        if let Some(name) = &result[1].name {
            assert_eq!(name, "example/v1/__init__.py");
        }
    }

    // #[test]
    // fn it_should_generate_py_init_files_with_no_opts() {
    //     let file_name = String::from("example/v1/foo_pb2.py");
    //     let binding: Option<py_package::PyPackageOptions> = None;
    //     let result: Vec<_> = generate_py_init_files(&file_name, &binding).collect();
    //     assert!(result.is_empty());
    // }
}
