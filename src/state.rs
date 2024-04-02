use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum StateName {
    Pending,
    Running,
    Completed,
    Failed,
}

impl ToString for StateName {
    fn to_string(&self) -> String {
        match self {
            StateName::Pending => "Pending".to_string(),
            StateName::Running => "Running".to_string(),
            StateName::Completed => "Completed".to_string(),
            StateName::Failed => "Failed".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub current_state: StateName,
    pub result: Option<String>,
    pub message: Option<String>,
}

impl State {
    pub fn new() -> Self {
        State {
            current_state: StateName::Pending,
            result: None,
            message: None,
        }
    }

    pub fn set(&mut self, new_state: StateName) {
        self.current_state = new_state;
    }

    pub fn set_result(&mut self, result: Option<String>) {
        self.result = result;
    }

    pub fn set_message(&mut self, message: Option<String>) {
        self.message = message;
    }
}
