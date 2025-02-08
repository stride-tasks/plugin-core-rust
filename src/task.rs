use chrono::{DateTime, Utc};

pub type Date = DateTime<Utc>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Annotation {
    pub entry: Date,
    pub description: String,
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type TagIndex = u32;
pub type ProjectIndex = u32;
pub type PriorityIndex = u32;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TaskStatus {
    #[default]
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "recurring")]
    Recurring,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "complete")]
    Complete,
}

impl TaskStatus {
    pub fn is_pending(&self) -> bool {
        *self == TaskStatus::Pending
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TaskPriority {
    #[default]
    #[serde(rename = "H")]
    H,
    #[serde(rename = "M")]
    M,
    #[serde(rename = "L")]
    L,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub uuid: Uuid,
    pub title: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "TaskStatus::is_pending")]
    pub status: TaskStatus,

    #[serde(default)]
    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub active: bool,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<Date>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<Date>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectIndex>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagIndex>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<Annotation>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TaskPriority>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<Date>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub depends: Vec<Uuid>,
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub uda: HashMap<String, String>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            uuid: Uuid::now_v7(),
            status: TaskStatus::Pending,
            title: String::new(),
            active: false,
            modified: None,
            due: None,
            project: None,
            tags: Vec::new(),
            annotations: Vec::new(),
            priority: None,
            wait: None,
            depends: Vec::new(),
            uda: HashMap::new(),
        }
    }
}

impl Task {
    #[must_use]
    pub fn entry(&self) -> Date {
        let timestamp = self
            .uuid
            .get_timestamp()
            .expect("uuid is v7 so this should not fail");
        let (secs, nsecs) = timestamp.to_unix();

        #[allow(clippy::cast_possible_wrap)]
        DateTime::from_timestamp(secs as i64, nsecs).expect("uuidv7 has a valid timestamp")
    }
}

impl TaskPriority {
    pub fn as_str(self) -> &'static str {
        match self {
            TaskPriority::H => "H",
            TaskPriority::M => "M",
            TaskPriority::L => "L",
        }
    }
}
