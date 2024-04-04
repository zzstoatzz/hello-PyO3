use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
mod client;
mod types;
use client::PrefectClient;
use types::{State, StateType, TaskRun};

#[pyfunction]
fn run_python_function(py: Python, func: PyObject, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<PyObject> {
    let task_name = func.getattr(py, "__name__")?;
    let mut state = State::new(StateType::Pending);

    let task_run_create = TaskRun {
        id: None,
        name: task_name.to_string(),
        flow_run_id: None,
        task_key: "example_task_key".to_string(),
        dynamic_key: "example_dynamic_key".to_string(),
        cache_key: None,
        cache_expiration: None,
        task_version: None,
        empirical_policy: None,
        tags: None,
        task_inputs: None,
        state: state.clone(),
    };

    let client = PrefectClient::new();
    let task_run = match client.create_task_run(&task_run_create) {
        Ok(task_run) => task_run,
        Err(e) => {
            state.state_type = StateType::Failed;
            state.set_message(format!("Failed to create task run: {}", e));
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to create task run: {}", e)));
        }
    };

    // print the name of the function and that it was moved into Running state
    println!("Running function: {}", task_name);

    // Update the state to Running
    state.state_type = StateType::Running;
    if let Err(e) = client.set_task_run_state(&task_run, &state) {
        state.state_type = StateType::Failed;
        state.set_message(format!("Failed to set task run state: {}", e));
    }

    // Execute the Python function
    let result = match kwargs {
        Some(kwargs) => func.call(py, args, Some(kwargs)),
        None => func.call(py, args, None),
    };

    // Check the result and update the state accordingly
    match result {
        Ok(_res) => {
            state.state_type = StateType::Completed;
            state.set_data(task_run.state.data.unwrap_or_default());
        }
        Err(e) => {
            state.state_type = StateType::Failed;
            state.set_message(e.to_string());
        }
    }

    // Set the final task run state
    if let Err(e) = client.set_task_run_state(&task_run, &state) {
        state.state_type = StateType::Failed;
        state.set_message(format!("Failed to set task run state: {}", e));
    }

    // Create a Python dictionary to represent the state
    let state_dict = PyDict::new(py);
    state_dict.set_item("type", state.state_type.to_string())?;
    if let Some(name) = &state.name {
        state_dict.set_item("name", name)?;
    }
    if let Some(data) = &state.data {
        state_dict.set_item("data", data)?;
    }
    if let Some(message) = &state.message {
        state_dict.set_item("message", message)?;
    }

    Ok(state_dict.into())
}

#[pymodule]
fn reeefect(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_python_function, m)?)?;
    Ok(())
}