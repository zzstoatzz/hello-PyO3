import functools
import uuid

import reeefect
from pydantic import TypeAdapter


def rust_task(func):
    """Decorator to ensure the function output is JSON serialized using TypeAdapter."""

    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        result = func(*args, **kwargs)
        return TypeAdapter(dict).dump_json(result).decode("utf-8")

    return wrapper


@rust_task
def python_function_that_returns_JSON(x, y, z=10):
    result = x + y + z
    print(f"result from Python: {result}\n")
    return {"x": x, "y": y, "z": z, "result": result}


def bad_python_function(x, y):
    print("PYTHON: we're about to fail\n")
    raise ValueError("This is a bad Python function")


# Call the Rust function with the Python function and arguments
result = reeefect.run_python_function(
    func=python_function_that_returns_JSON,
    args=(5, 7),
    kwargs={"z": 3},
    options={"name": "this should work", "dynamic_key": str(uuid.uuid4())},
)

print(f"Result from Rust: {result}")

print("--------------------")
print("Now we're going to call a bad Python function\n")

# Call the Rust function with the Python function and arguments
result = reeefect.run_python_function(
    bad_python_function,
    args=(5, 7),
    options={"name": "we gonna fail", "dynamic_key": str(uuid.uuid4())},
)

print(f"Result from Rust: {result}")
