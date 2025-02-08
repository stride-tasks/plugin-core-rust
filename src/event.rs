use serde::{Deserialize, Serialize};

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
