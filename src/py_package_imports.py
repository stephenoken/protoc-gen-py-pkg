import importlib
import pkgutil
import inspect
import sys
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Get the current package
package = sys.modules[__name__]
package_prefix = package.__name__ + "."
__all__ = []


# Define which classes to include (e.g., only classes with uppercase names)
def should_include(name, obj):
    return inspect.isclass(obj) and name[0].isupper() and not name.startswith("_")


# Walk through all submodules
for _, name, is_pkg in pkgutil.walk_packages(package.__path__, package_prefix):
    try:
        module = importlib.import_module(name)
        # Find matching classes
        for attr_name, attr_value in inspect.getmembers(module):
            if should_include(attr_name, attr_value):
                globals()[attr_name] = attr_value
                __all__.append(attr_name)
    except ImportError as error:
        logger.warning(f"Failed to import module {name}: {error}")
        