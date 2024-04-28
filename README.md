### hello-PyO3

```python
(hello-PyO3) nate :: ~/github.com/zzstoatzz/hello-PyO3 ‹main›
» python example.py
Creating task run: url=https://api.prefect.cloud/api/accounts/xxx/workspaces/xxx/task_runs/, payload=TaskRun { id: None, name: "my_python_function_that_returns_JSON", flow_run_id: None, task_key: "example_task_key", dynamic_key: "my_python_function_that_returns_JSON", cache_key: None, cache_expiration: None, empirical_policy: None, task_inputs: None, task_version: None, tags: None, state: State { state_type: Pending, name: None, message: None, data: None } }

Running function: my_python_function_that_returns_JSON

Setting task run state: url=https://api.prefect.cloud/api/accounts/xxx/workspaces/xxx/task_runs/c619fb2d-e2ec-4449-abeb-1c5901accfbb/state/, payload=State { state_type: Running, name: None, message: None, data: None }

result from Python: 15

Setting task run state: url=https://api.prefect.cloud/api/accounts/xxx/workspaces/xxx/task_runs/c619fb2d-e2ec-4449-abeb-1c5901accfbb/state/, payload=State { state_type: Completed, name: None, message: None, data: Some("{'x': 5, 'y': 7, 'z': 3, 'result': 15}") }

Result from Rust: {'type': 'Completed', 'data': "{'x': 5, 'y': 7, 'z': 3, 'result': 15}"}



Creating task run: url=https://api.prefect.cloud/api/accounts/xxx/workspaces/xxx/task_runs/, payload=TaskRun { id: None, name: "my_bad_python_function", flow_run_id: None, task_key: "example_task_key", dynamic_key: "my_bad_python_function", cache_key: None, cache_expiration: None, empirical_policy: None, task_inputs: None, task_version: None, tags: None, state: State { state_type: Pending, name: None, message: None, data: None } }

Running function: my_bad_python_function

Setting task run state: url=https://api.prefect.cloud/api/accounts/xxx/workspaces/xxx/task_runs/5fabd2a4-1a7d-4c50-b941-68877f34ce62/state/, payload=State { state_type: Running, name: None, message: None, data: None }

PYTHON: we're about to fail

Setting task run state: url=https://api.prefect.cloud/api/accounts/xxx/workspaces/xxx/task_runs/5fabd2a4-1a7d-4c50-b941-68877f34ce62/state/, payload=State { state_type: Failed, name: None, message: Some("ValueError: This is a bad Python function"), data: None }

Result from Rust: {'type': 'Failed', 'message': 'ValueError: This is a bad Python function'}
```