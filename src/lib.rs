
pub mod protos;
use std::path::{self, Path};

use protobuf::plugin::code_generator_response::File;
use protos::py_package;

pub fn generate_py_init_files(opts: &Option<py_package::PyPackageOptions>) -> impl Iterator<Item = File> {
    // Creates an iterator with 0 or 1 items based on whether `opts` is `Some` or `None`.
    opts
    // Using as_ref to convert the Option into an Option<&PyPackageOptions>.
    // This allows us to avoid moving the value out of the Option.
    .as_ref()
    .into_iter()
    // Using flat map to iterate over the option and generate an iterator of `File` objects.
    .flat_map(|opt| {
        let components: Vec<_> = opt.name.split('.').collect();
        // Use scan to accumulate path components while yielding each one. 
        (0..components.len()).scan(String::new(), move |path_so_far, index | {
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
            file.set_content(String::from("# Generated Python package init file"));
            Some(file)
        })
    })

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_should_generate_py_init_files() {
        let mut opts = py_package::PyPackageOptions::new();
        opts.name = String::from("example.v1");
        let binding = Some(opts);
        let result: Vec<_> = generate_py_init_files(&binding).collect();
        assert_eq!(result.len(), 2);
        if let Some(name) = &result[0].name {
            assert_eq!(name, "example/__init__.py");
        }
        if let Some(name) = &result[1].name {
            assert_eq!(name, "example/v1/__init__.py");
        }
    }

}