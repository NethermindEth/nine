use n9_core::Prompt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type TaskId = u64;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct TaskInfo {
    pub id: TaskId,
    pub repeat: bool,
    pub interval_sec: u64,
    pub prompt: String,
}

/// Task List Display Tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct TasksList {}

impl Prompt for TasksList {
    type Output = Vec<TaskInfo>;

    fn description() -> &'static str {
        "This tool displays a list of all created tasks, providing an overview of each task's ID,
        repetition interval, and query. It allows users to easily monitor the status and details
        of all ongoing or scheduled tasks. The interface provides a clear and organized view of
        all tasks, enabling efficient task management and oversight."
    }
}

/// Task Creation Tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct TaskAdd {
    /// A boolean flag indicating whether the task should repeat after each interval.
    /// If `false` the task runs only once.
    pub repeat: bool,
    /// The time interval (in seconds) between each task repetition.
    pub interval_sec: u64,
    /// The request or action that will be sent when the task is triggered.
    pub prompt: String,
}

impl Prompt for TaskAdd {
    type Output = TaskId;

    fn description() -> &'static str {
        "This tool is responsible for creating new tasks. Each task is assigned a unique numerical ID,
        which follows a sequential order. Additionally, users can specify the number of seconds after
        which the task should repeat. The tool also allows the definition of the task's query, which
        is the specific request or action that the task will send. This tool is essential for initiating
        tasks and defining their recurring behavior and actions."
    }
}

/// Task Removal Tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct TaskDel {
    /// The unique numerical identifier of the task to be deleted from the system.
    pub id: TaskId,
}

impl Prompt for TaskDel {
    type Output = bool;

    fn description() -> &'static str {
        "This tool allows users to remove tasks from the system. By specifying the task's ID,
        the tool cancels the task, stopping it from repeating and executing any further.
        This feature is crucial for maintaining an organized task list and ensuring that
        unnecessary or completed tasks are properly managed and deleted from the system."
    }
}
