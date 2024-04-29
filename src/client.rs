use crate::types::{OrchestrationResult, State, TaskRun};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;

#[derive(Debug)]
pub struct PrefectClientError {
    pub status_code: StatusCode,
    pub message: String,
}

impl fmt::Display for PrefectClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Request failed with status code {}: {}",
            self.status_code.as_u16(),
            self.message
        )
    }
}

impl std::error::Error for PrefectClientError {}

pub struct PrefectClient {
    client: Client,
    base_url: String,
}

impl PrefectClient {
    pub fn new() -> Self {
        let api_url = env::var("PREFECT_API_URL").expect("PREFECT_API_URL must be set");
        let api_key = env::var("PREFECT_API_KEY").expect("PREFECT_API_KEY must be set");

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder().default_headers(headers).build().unwrap();

        PrefectClient {
            client,
            base_url: api_url,
        }
    }

    fn handle_response<T: for<'de> Deserialize<'de>>(
        &self,
        response: reqwest::Result<reqwest::blocking::Response>,
    ) -> Result<T, PrefectClientError> {
        let response = response.map_err(|err| PrefectClientError {
            status_code: err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            message: format!("Network error: {}", err),
        })?;

        let status_code = response.status();
        let raw_body = response
            .text()
            .unwrap_or_else(|_| "Failed to read response body".to_string());

        if status_code.is_success() {
            serde_json::from_str::<T>(&raw_body).map_err(|err| PrefectClientError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("JSON parsing error: {} - Response: {}", err, raw_body),
            })
        } else {
            Err(PrefectClientError {
                status_code,
                message: format!("API error ({}): {}", status_code, raw_body),
            })
        }
    }

    pub fn create_task_run(&self, task_run: &TaskRun) -> Result<TaskRun, PrefectClientError> {
        println!("Creating task run {:#?}", task_run);
        let url = format!("{}/task_runs/", self.base_url);
        let response = self.client.post(url).json(task_run).send();
        self.handle_response(response)
    }

    pub fn set_task_run_state(
        &self,
        task_run: &TaskRun,
        state: &State,
    ) -> Result<OrchestrationResult, PrefectClientError> {
        println!("Setting task run state to {:#?}", state.state_type);
        if let Some(task_run_id) = &task_run.id {
            let url = format!("{}/task_runs/{}/set_state/", self.base_url, task_run_id);
            let payload = serde_json::json!({
                "state": state,
            });
            let response = self.client.post(url).json(&payload).send();
            self.handle_response(response)
        } else {
            Err(PrefectClientError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Task run ID is missing, cannot set state".to_string(),
            })
        }
    }
}
