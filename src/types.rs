use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StateType {
    Scheduled,
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
    Crashed,
    Paused,
    Cancelling,
}

impl core::fmt::Display for StateType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct State<T>
where
    T: serde::Serialize + std::clone::Clone,
{
    #[serde(rename = "type")]
    pub state_type: StateType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}
impl<T> State<T>
where
    T: serde::Serialize + std::clone::Clone,
{
    pub fn new(state_type: StateType) -> Self {
        State {
            state_type,
            name: None,
            message: None,
            data: None,
        }
    }

    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }

    pub fn set_data(&mut self, data: T) {
        self.data = Some(data);
    }
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct TaskRun<T>
where
    T: serde::Serialize + std::clone::Clone + std::fmt::Debug,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_run_id: Option<String>,
    pub task_key: String,
    pub dynamic_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_expiration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empirical_policy: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_inputs: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    pub state: State<T>,
}
