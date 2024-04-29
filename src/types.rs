use crate::dates::datetime_iso8601;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug};
use uuid::Uuid;

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrchestrationResultStatus {
    Accept,
    Reject,
    Abort,
    Wait,
}

impl core::fmt::Display for StateType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StateDetails {
    pub cache_expiration: Option<String>,
    pub cache_key: Option<String>,
    pub child_flow_run_id: Option<Uuid>,
    pub flow_run_id: Option<Uuid>,
    pub pause_key: Option<String>,
    pub pause_reschedule: Option<bool>,
    pub pause_timeout: Option<String>,
    pub refresh_cache: Option<bool>,
    pub retriable: Option<bool>,
    pub run_input_keyset: Option<String>,
    pub scheduled_time: Option<DateTime<Utc>>,
    pub task_parameters_id: Option<Uuid>,
    pub task_run_id: Option<Uuid>,
    pub transition_id: Option<Uuid>,
    pub untrackable_result: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct State {
    #[serde(rename = "type")]
    pub state_type: StateType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(with = "datetime_iso8601", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}
impl State {
    pub fn new(state_type: StateType) -> Self {
        State {
            state_type,
            name: None,
            message: None,
            data: None,
            timestamp: None,
        }
    }

    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }

    pub fn set_data(&mut self, data: Value) {
        self.data = Some(data);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmpiricalPolicy {
    pub max_retries: Option<i64>,
    pub retries: Option<i64>,
    pub retry_delay: Option<String>,
    pub retry_delay_seconds: Option<f64>,
    pub retry_jitter_factor: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaskRun {
    pub name: String,
    pub flow_run_id: Option<Uuid>,
    pub task_key: String,
    pub dynamic_key: String,
    pub cache_key: Option<String>,
    pub cache_expiration: Option<String>,
    pub task_version: Option<String>,
    pub state: State,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empirical_policy: Option<EmpiricalPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_inputs: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(with = "datetime_iso8601", skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(with = "datetime_iso8601", skip_serializing_if = "Option::is_none")]
    pub updated: Option<DateTime<Utc>>,
    #[serde(with = "datetime_iso8601", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(with = "datetime_iso8601", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_run_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_run_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_count: Option<i64>,
    #[serde(with = "datetime_iso8601", skip_serializing_if = "Option::is_none")]
    pub expected_start_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_start_time_delta: Option<f64>,
    #[serde(with = "datetime_iso8601", skip_serializing_if = "Option::is_none")]
    pub next_scheduled_start_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_run_run_count: Option<i64>,
}

impl Default for TaskRun {
    fn default() -> Self {
        TaskRun {
            id: None,
            name: String::new(),
            flow_run_id: None,
            task_key: String::new(),
            dynamic_key: String::new(),
            cache_key: None,
            cache_expiration: None,
            empirical_policy: None,
            task_inputs: None,
            task_version: None,
            tags: None,
            state: State {
                state_type: StateType::Pending,
                name: None,
                message: None,
                data: None,
                timestamp: None,
            },
            created: None,
            updated: None,
            start_time: None,
            end_time: None,
            estimated_run_time: None,
            total_run_time: None,
            run_count: None,
            expected_start_time: None,
            estimated_start_time_delta: None,
            next_scheduled_start_time: None,
            flow_run_run_count: None,
        }
    }
}

#[derive(Debug, Default, Serialize, Clone)]
pub struct TaskRunOptions {
    pub name: Option<String>,
    pub cache_key: Option<String>,
    pub dynamic_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OrchestrationResult {
    pub state: State,
    pub status: OrchestrationResultStatus,
}
