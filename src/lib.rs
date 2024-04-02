use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

mod client;
mod state;

use client::PrefectClient;
use state::{State, StateName};

#[pyfunction]
fn run_python_function(py: Python, func: PyObject, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<PyObject> {
    let mut state = State::new();
    state.set(StateName::Pending);

    let client = PrefectClient::new("http://localhost:4200");

    // Create a task run before executing the Python function
    let task_run_id = match client.create_task_run("mock_task_id") {
        Ok(id) => id,
        Err(e) => {
            state.set(StateName::Failed);
            state.set_message(Some(format!("Failed to create task run: {}", e)));
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to create task run: {}", e)));
        }
    };

    // print the name of the function and that it was moved into Running state
    println!("Running function: {}", func.getattr(py, "__name__")?);

    // Update the state to Running
    state.set(StateName::Running);
    if let Err(e) = client.set_task_run_state(&task_run_id, &state.current_state) {
        state.set(StateName::Failed);
        state.set_message(Some(format!("Failed to set task run state: {}", e)));
    }

    // Execute the Python function
    let result = match kwargs {
        Some(kwargs) => func.call(py, args, Some(kwargs)),
        None => func.call(py, args, None),
    };

    // Check the result and update the state accordingly
    match result {
        Ok(res) => {
            state.set(StateName::Completed);
            state.set_result(Some(res.to_string()));
        }
        Err(e) => {
            state.set(StateName::Failed);
            state.set_message(Some(e.to_string()));
        }
    }

    // Set the final task run state
    if let Err(e) = client.set_task_run_state(&task_run_id, &state.current_state) {
        state.set(StateName::Failed);
        state.set_message(Some(format!("Failed to set task run state: {}", e)));
    }

    // Create a Python dictionary to represent the state
    let state_dict = PyDict::new(py);
    state_dict.set_item("name", state.current_state.to_string())?;
    
    if let Some(result) = &state.result {
        state_dict.set_item("result", result)?;
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