use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
mod client;
mod dates;
mod types;
use client::PrefectClient;
use types::{State, StateType, TaskRun, TaskRunOptions};

fn create_state_dict(py: Python, state: &State) -> PyResult<Py<PyDict>> {
    let state_dict = PyDict::new_bound(py);
    state_dict.set_item("type", state.state_type.to_string())?;
    if let Some(name) = &state.name {
        state_dict.set_item("name", name)?;
    }
    if let Some(data) = &state.data {
        state_dict.set_item("data", data.to_string())?;
    }
    if let Some(message) = &state.message {
        state_dict.set_item("message", message)?;
    }
    Ok(state_dict.into())
}

#[pyfunction]
fn run_python_function(
    py: Python,
    func: PyObject,
    args: &pyo3::Bound<'_, PyTuple>, // bound to the python GIL lifetime?
    kwargs: Option<&pyo3::Bound<'_, PyDict>>,
    options: Option<HashMap<String, String>>,
) -> PyResult<PyObject> {
    let task_name: String = func.getattr(py, "__name__")?.extract(py).unwrap();
    let mut state = State::new(StateType::Pending);

    let task_run_options = options.map(|opts| {
        let mut task_run_opts = TaskRunOptions::default();
        if let Some(name) = opts.get("name") {
            task_run_opts.name = Some(name.clone());
        }
        if let Some(cache_key) = opts.get("cache_key") {
            task_run_opts.cache_key = Some(cache_key.clone());
        }
        if let Some(dynamic_key) = opts.get("dynamic_key") {
            task_run_opts.dynamic_key = Some(dynamic_key.clone());
        }
        task_run_opts
    });

    let task_run_create = TaskRun {
        name: task_run_options
            .as_ref()
            .and_then(|opts| opts.name.clone())
            .unwrap_or(task_name.clone()),
        dynamic_key: task_name.clone(),
        task_key: task_name.clone(),
        cache_key: task_run_options
            .as_ref()
            .and_then(|opts| opts.cache_key.clone()),
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

    println!(
        "Running function: {} with parameters: {:?}\n",
        task_name, args
    );
    state.state_type = StateType::Running;
    if let Err(e) = client.set_task_run_state(&task_run, &state) {
        let error_message = format!("Failed to set task run state: {}", e);
        state.state_type = StateType::Crashed;
        state.set_message(error_message);
        return Ok(create_state_dict(py, &state)?.into_py(py));
    }

    let result = match kwargs {
        Some(kwargs) => func.call_bound(py, args, Some(kwargs)),
        None => func.call_bound(py, args, None),
    };

    match result {
        Ok(res) => {
            state.state_type = StateType::Completed;
            state.set_data(serde_json::from_str(&res.to_string()).unwrap());
        }
        Err(e) => {
            state.state_type = StateType::Failed;
            state.set_message(e.to_string());
        }
    }

    if let Err(e) = client.set_task_run_state(&task_run, &state) {
        state.state_type = StateType::Crashed;
        state.set_message(format!("Could not set task run state: {}", e));
        return Ok(create_state_dict(py, &state)?.into_py(py));
    }

    Ok(create_state_dict(py, &state)?.into_py(py))
}

#[pymodule]
fn reeefect(_py: Python, module: &pyo3::Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(run_python_function, module)?)?;
    Ok(())
}
