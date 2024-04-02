import reeefect


def my_python_function(x, y, z=10):
    result = x + y + z
    print(f"PYTHON: {result=!r}")
    return result


def my_bad_python_function(x, y):
    print("PYTHON: we're about to fail")
    raise ValueError("This is a bad Python function")


# Call the Rust function with the Python function and arguments
result = reeefect.run_python_function(my_python_function, args=(5, 7), kwargs={"z": 3})

print(f"Result from Rust: {result}")

print("\n\n")

# Call the Rust function with the Python function and arguments
result = reeefect.run_python_function(my_bad_python_function, args=(5, 7))
