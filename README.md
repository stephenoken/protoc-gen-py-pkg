# protoc-gen-py-pkg

protoc-gen-py-pkg is a plugin for ProtocolBuffer compiler. It generates `__init__.py` files from 
proto messages to ease the process for creating python packages.

## Installation

```sh
cargo install --git https://github.com/stephenoken/protoc-gen-py-pkg
```

## Usage

protoc --py-pk_out=path/to/out/dir foo.proto
`protoc` and `protoc-gen-py-pkg` commands must be found in $PATH.

The proto messages must have the `option (py_package.py_package_opts).enable = true;` set.

To enable "cleaner" import of python compiled proto messages you can set `enable_top_level_imports`
to be `true` as file level option. If set, and with the following protoc flag
`--py-pkg_out=examples/src/foo` the import statement would be `from examples import Foo`. This assumes that
examples is the root module of your python object.

If you support multiple versions of your generated code (I.E v1, v1, etc) you can also set
`enable_versioned_imports` to be `true` as file level option. If set true the top level imports
will append the version number to the top level import. For example
`from examples.v1.foo_pb2 import Foo` can be imported as `from examples import Foo_v1`.

Note: that you will also need to include `py_package.proto` as part of your protobuf build.

### Example

Suppose that we have the following foo.proto.

```proto
syntax = "proto3";
import "google/protobuf/descriptor.proto";
import "py_package.proto";

option (py_package.py_package_opts).enable = true;
option (py_package.py_package_opts).enable_top_level_imports = true;

message Foo {
  string name = 1;
  message Bar {
    string name = 1;
  } 
  Bar bar = 2;
}

message Bar {
  string name = 1;
}
```

`protoc --py-pkg_out=. foo.proto` will generate a file named `__init__.py`.
