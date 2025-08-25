import importlib
import pkgutil
import inspect
import sys
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

package = sys.modules[__name__]
package_prefix = package.__name__ + "."
__all__ = []

def should_include(name, obj):
    return inspect.isclass(obj) and name[0].isupper() and not name.startswith("_")

def get_version_suffix(modname):
    # Extract v0, v1, etc. from module name
    parts = modname.split('.')
    for part in parts:
        if part.startswith('v') and part[1:].isdigit():
            return '_' + part
    return ''

# Walk through all submodules
for _, modname, is_pkg in pkgutil.walk_packages(package.__path__, package_prefix):
    try:
        module = importlib.import_module(modname)
        suffix = get_version_suffix(modname)
        for attr_name, attr_value in inspect.getmembers(module):
            if should_include(attr_name, attr_value):
                new_name = f"{attr_name}{suffix}" if suffix else attr_name
                globals()[new_name] = attr_value
                __all__.append(new_name)
    except ImportError:
        logger.warning(f"Failed to import module {name}: {error}")
