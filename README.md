### hello-PyO3

```python
(hello-PyO3) nate :: ~/github.com/zzstoatzz/hello-PyO3 ‹main›
» python example.py
Running function: my_python_function
Setting task run state: task_run_id=mock_task_id, state=Running
PYTHON: result=15
Setting task run state: task_run_id=mock_task_id, state=Completed
Result from Rust: {'name': 'Completed', 'result': '15'}



Running function: my_bad_python_function
Setting task run state: task_run_id=mock_task_id, state=Running
PYTHON: we're about to fail
Setting task run state: task_run_id=mock_task_id, state=Failed
```