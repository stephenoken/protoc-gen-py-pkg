import importlib
import pkgutil
import inspect
import sys

# Get the current package
package = sys.modules[__name__]
package_prefix = package.__name__ + '.'
__all__ = []

# Define which classes to include (e.g., only classes with uppercase names)
def should_include(name, obj):
    print(f"Importing {name}")
    return inspect.isclass(obj) and name[0].isupper() and not name.startswith('_')

# Walk through all submodules
for _, name, is_pkg in pkgutil.walk_packages(package.__path__, package_prefix):
    try:
        module = importlib.import_module(name)
        print(f"Processing module: {name}")
        # Find matching classes
        for attr_name, attr_value in inspect.getmembers(module):
            print(f"Checking {attr_name} in {name}")
            if should_include(attr_name, attr_value):
                globals()[attr_name] = attr_value
                __all__.append(attr_name)
    except ImportError:
        pass