use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
mod client;
mod types;
use client::PrefectClient;
use types::{State, StateType, TaskRun};

fn create_state_dict<'py>(py: Python<'py>, state: &State<String>) -> PyResult<&'py PyDict> {
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
    Ok(state_dict)
}

#[pyfunction]
fn run_python_function(py: Python, func: PyObject, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<PyObject> {
    let task_name: String = func.getattr(py, "__name__")?.extract(py).unwrap();
    let mut state = State::<String>::new(StateType::Pending);
    let task_run_create = TaskRun {
        name: task_name.clone(),
        dynamic_key: task_name.clone(),
        task_key: task_name.clone(),
        state: state.clone(),
        ..TaskRun::default()
    };

    let client = PrefectClient::new();
    let task_run = match client.create_task_run(&task_run_create) {
        Ok(task_run) => task_run,
        Err(e) => {
            let error_message = format!("Failed to submit task run: {}", e);
            println!("{}\n", error_message);
            state.state_type = StateType::Crashed;
            return Ok(create_state_dict(py, &state)?.into_py(py));
        }
    };

    println!("Running function: {} with parameters: {:?}\n", task_name, args);
    state.state_type = StateType::Running;
    if let Err(e) = client.set_task_run_state(&task_run, &state) {
        let error_message = format!("Failed to set task run state: {}", e);
        state.state_type = StateType::Failed;
        state.set_message(error_message);
        return Ok(create_state_dict(py, &state)?.into_py(py));
    }

    let result = match kwargs {
        Some(kwargs) => func.call(py, args, Some(kwargs)),
        None => func.call(py, args, None),
    };

    match result {
        Ok(res) => {
            state.state_type = StateType::Completed;
            state.set_data(res.to_string());
        }
        Err(e) => {
            state.state_type = StateType::Failed;
            state.set_message(e.to_string());
        }
    }

    if let Err(e) = client.set_task_run_state(&task_run, &state) {
        let error_message = format!("Failed to set task run state: {}", e);
        state.state_type = StateType::Crashed;
        state.set_message(error_message);
        return Ok(create_state_dict(py, &state)?.into_py(py));
    }

    Ok(create_state_dict(py, &state)?.into_py(py))
}

#[pymodule]
fn reeefect(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_python_function, m)?)?;
    Ok(())

}