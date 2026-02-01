use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Location {
  pub xname: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "deputyKey")]
  pub deputy_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Operation {
  #[serde(rename = "on")]
  On,
  #[serde(rename = "off")]
  Off,
  #[serde(rename = "soft-off")]
  SoftOff,
  #[serde(rename = "soft-restart")]
  SoftRestart,
  #[serde(rename = "hard-restart")]
  HardRestart,
  #[serde(rename = "init")]
  Init,
  #[serde(rename = "force-off")]
  ForceOff,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transition {
  pub operation: Operation,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "taskDeadlineMinutes")]
  pub task_deadline_minutes: Option<usize>,
  pub location: Vec<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCounts {
  pub total: usize,
  pub new: usize,
  pub in_progress: usize,
  pub failed: usize,
  pub succeeded: usize,
  pub un_supported: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
  pub xname: String,
  #[serde(rename = "taskStatus")]
  pub task_status: String,
  #[serde(rename = "taskStatusDescription")]
  pub task_status_description: String,
  pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransitionResponse {
  #[serde(rename = "transitionID")]
  pub transition_id: String,
  #[serde(rename = "createTime")]
  pub create_time: String,
  #[serde(rename = "automaticExpirationTime")]
  pub automatic_expiration_time: String,
  #[serde(rename = "transitionStatus")]
  pub transition_status: String,
  pub operation: Operation,
  #[serde(rename = "taskCounts")]
  pub task_counts: TaskCounts,
  pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransitionResponseList {
  pub transitions: Vec<TransitionResponse>,
}
