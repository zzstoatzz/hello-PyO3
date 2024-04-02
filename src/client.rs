use reqwest::Client;
use crate::state::StateName;

pub struct PrefectClient {
    client: Client,
    base_url: String,
}

impl PrefectClient {
    pub fn new(base_url: &str) -> Self {
        PrefectClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub fn create_task_run(&self, task_id: &str) -> Result<String, reqwest::Error> {
        // TODO: Implement the logic to create a task run using the Prefect API
        Ok(task_id.to_string())
    }

    pub fn set_task_run_state(&self, task_run_id: &str, state: &StateName) -> Result<(), reqwest::Error> {
        // TODO: Implement the logic to set the task run state using the Prefect API
        println!("Setting task run state: task_run_id={}, state={:?}", task_run_id, state);
        Ok(())
    }
}