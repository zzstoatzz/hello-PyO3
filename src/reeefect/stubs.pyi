from typing import Any, Callable, Dict, ParamSpec, Tuple, TypeVar

P = ParamSpec("P")
R = TypeVar("R")

def run_python_function(
    func: Callable[P, R], args: Tuple, kwargs: Dict[str, Any] | None = None
) -> Dict[str, Any]: ...
