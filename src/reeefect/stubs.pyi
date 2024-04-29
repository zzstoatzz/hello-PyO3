from typing import Any, Callable, Dict, ParamSpec, Tuple, TypedDict, TypeVar

P = ParamSpec("P")
R = TypeVar("R")

class TaskRunOptions(TypedDict):
    name: str
    cache_key: str
    dynamic_key: str

def run_python_function(
    func: Callable[P, R],
    args: Tuple,
    kwargs: Dict[str, Any] | None = None,
    options: TaskRunOptions | None = None,
) -> Dict[str, Any]: ...
