use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::task::Task;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Event {
    TaskCreate {
        task: Option<Box<Task>>,
    },
    TaskRemove {
        task: Option<Box<Task>>,
    },
    TaskModified {
        current: Option<Box<Task>>,
        previous: Option<Box<Task>>,
    },
    TaskSync,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EmitEvent {
    TaskCreate { task: Task },
    TaskRemove { task: Uuid },
    TaskModify { task: Task },
    TaskSync,
}
