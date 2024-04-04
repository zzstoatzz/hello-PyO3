use std::env;
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

    pub fn create_task_run<T>(&self, payload: &TaskRun<T>) -> Result<TaskRun<T>, reqwest::Error>
    where
        T: serde::Serialize + std::clone::Clone + std::fmt::Debug,
    {
        let url = format!("{}/task_runs/", self.base_url);

        // debug print the URL and payload
        println!("Creating task run: url={}, payload={:?}", url, payload);

        let response = self.client.post(&url)
            .json(payload)
            .send()?;

        let whatever: String = response.text()?;
        Ok(TaskRun {
            id: Some(whatever),
            name: payload.name.clone(),
            flow_run_id: payload.flow_run_id.clone(),
            task_key: payload.task_key.clone(),
            dynamic_key: payload.dynamic_key.clone(),
            cache_key: payload.cache_key.clone(),
            cache_expiration: payload.cache_expiration.clone(),
            task_version: payload.task_version.clone(),
            empirical_policy: payload.empirical_policy.clone(),
            tags: payload.tags.clone(),
            task_inputs: payload.task_inputs.clone(),
            state: payload.state.clone(),
        })
    }

    pub fn set_task_run_state(&self, task_run: &TaskRun<()>, state: &State<()>) -> Result<(), reqwest::Error> {
        let url = format!("{}/task_runs/{:?}/state/", self.base_url, task_run.id);

        // debug print the URL and payload
        println!("Setting task run state: url={}, payload={:?}", url, state);

        let response = self.client.post(&url)
            .json(state)
            .send()?;

        let _whatever: String = response.text()?;
        Ok(())
    }
}