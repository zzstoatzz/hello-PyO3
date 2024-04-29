### hello-PyO3

```python
(hello-PyO3) nate :: ~/github.com/zzstoatzz/hello-PyO3 ‹main›
» python example.py
Creating task run TaskRun {
    name: "this should work",
    flow_run_id: None,
    task_key: "python_function_that_returns_JSON",
    dynamic_key: "python_function_that_returns_JSON",
    cache_key: None,
    cache_expiration: None,
    task_version: None,
    state: State {
        state_type: Pending,
        name: None,
        message: None,
        data: None,
        timestamp: None,
    },
    id: None,
    empirical_policy: None,
    task_inputs: None,
    tags: None,
    created: None,
    updated: None,
    start_time: None,
    end_time: None,
    estimated_run_time: None,
    total_run_time: None,
    run_count: None,
    expected_start_time: None,
    estimated_start_time_delta: None,
    next_scheduled_start_time: None,
    flow_run_run_count: None,
}
Running function: python_function_that_returns_JSON with parameters: (5, 7)

Setting task run state to Running
result from Python: 15

Setting task run state to Completed
Result from Rust: {'type': 'Completed', 'data': '{"result":15,"x":5,"y":7,"z":3}'}
--------------------
Now we're going to call a bad Python function

Creating task run TaskRun {
    name: "we gonna fail",
    flow_run_id: None,
    task_key: "bad_python_function",
    dynamic_key: "bad_python_function",
    cache_key: None,
    cache_expiration: None,
    task_version: None,
    state: State {
        state_type: Pending,
        name: None,
        message: None,
        data: None,
        timestamp: None,
    },
    id: None,
    empirical_policy: None,
    task_inputs: None,
    tags: None,
    created: None,
    updated: None,
    start_time: None,
    end_time: None,
    estimated_run_time: None,
    total_run_time: None,
    run_count: None,
    expected_start_time: None,
    estimated_start_time_delta: None,
    next_scheduled_start_time: None,
    flow_run_run_count: None,
}
Running function: bad_python_function with parameters: (5, 7)

Setting task run state to Running
PYTHON: we're about to fail

Setting task run state to Failed
Result from Rust: {'type': 'Failed', 'message': 'ValueError: This is a bad Python function'}
```