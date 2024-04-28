use std::env;
use std::io::{Error, ErrorKind};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use crate::types::{State, TaskRun};


pub struct PrefectClient {
    client: reqwest::blocking::Client,
    base_url: String,
}

impl PrefectClient {
    pub fn new() -> Self {
        let api_url = env::var("PREFECT_API_URL").expect("PREFECT_API_URL must be set");
        let api_key = env::var("PREFECT_API_KEY").expect("PREFECT_API_KEY must be set");

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        PrefectClient {
            client,
            base_url: api_url,
        }
    }

    pub fn create_task_run<T>(&self, task_run: &TaskRun<T>) -> Result<TaskRun<T>, reqwest::Error>
    where
        T: serde::Serialize + std::clone::Clone + std::fmt::Debug,
    {
        let url = format!("{}/task_runs/", self.base_url);

        println!("Creating task run: url={}, payload={:?}", url, task_run);
        let response = self.client.post(&url)
            .json(task_run)
            .send()?;
    
        // Check if the response is a 200 OK
        if response.status().is_success() {
            let json_response: serde_json::Value = response.json()?;
    
            let task_run_id = json_response["id"].as_str().unwrap_or_default().to_string();
    
            Ok(TaskRun {
                id: Some(task_run_id),
                name: task_run.name.clone(),
                flow_run_id: task_run.flow_run_id.clone(),
                task_key: task_run.task_key.clone(),
                dynamic_key: task_run.dynamic_key.clone(),
                cache_key: task_run.cache_key.clone(),
                cache_expiration: task_run.cache_expiration.clone(),
                task_version: task_run.task_version.clone(),
                empirical_policy: task_run.empirical_policy.clone(),
                tags: task_run.tags.clone(),
                task_inputs: task_run.task_inputs.clone(),
                state: task_run.state.clone(),
            })
        } else {
            // If the response is not a 200 OK, return an error
            let error_message = format!("Could not create task run. Status code: {}", response.status());
            println!("{}", error_message);
            Err(reqwest::Error::from(response.error_for_status().unwrap_err()))
        }
    }

    
    pub fn set_task_run_state<T>(&self, task_run: &TaskRun<T>, state: &State<T>) -> Result<String, Error>
    where
        T: serde::Serialize + std::clone::Clone + std::fmt::Debug,
    {
        if let Some(task_run_id) = &task_run.id {
            let url = format!("{}/task_runs/{}/state/", self.base_url, task_run_id);
            // debug print the URL and payload
            println!("Setting task run state: url={}, payload={:?}", url, state);
            let response = self.client.post(&url)
                .json(state)
                .send();
            let result: String = response.unwrap().text().unwrap();
            Ok(result)
        } else {
            let error_message = "Cannot set task run state: task_run.id is None".to_string();
            println!("{}", error_message);
            Err(Error::new(ErrorKind::InvalidData, error_message))
        }
    }
}