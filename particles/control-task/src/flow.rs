use crate::tools::{TaskId, TaskInfo, TaskParameters};
use chrono::{DateTime, Duration, Utc};
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use ui9::names::Fqn;
use ui9_dui::{BareTracer, Flow, Listener, Publisher, Subscriber, Tracer, Unified};

pub type Deadline = DateTime<Utc>;

#[derive(Deref, DerefMut, From, Into)]
pub struct TasksSub {
    listener: Listener<Tasks>,
}

impl Subscriber for Tasks {
    type Driver = TasksSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct TasksPub {
    tracer: Tracer<Tasks>,
}

impl Publisher for Tasks {
    type Driver = TasksPub;
}

impl TasksPub {
    pub fn create(&self, id: TaskId, parameters: TaskParameters) -> TaskPub {
        TaskPub {
            id,
            duration: Duration::seconds(parameters.interval_sec as i64),
            tracer: self.bare_tracer(),
        }
    }
}

pub struct TaskPub {
    id: TaskId,
    duration: Duration,
    tracer: BareTracer<Tasks>,
}

impl TaskPub {
    pub fn add(&mut self, parameters: TaskParameters) {
        let event = TasksEvent::Add {
            id: self.id,
            deadline: Utc::now() + self.duration,
            parameters,
        };
        self.tracer.event(event);
    }

    pub fn update(&mut self) {
        let event = TasksEvent::Update {
            id: self.id,
            deadline: Utc::now() + self.duration,
        };
        self.tracer.event(event);
    }

    pub fn del(&mut self) {
        let event = TasksEvent::Del { id: self.id };
        self.tracer.event(event);
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TaskRecord {
    pub parameters: TaskParameters,
    pub deadline: Deadline,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Tasks {
    pub tasks: BTreeMap<TaskId, TaskRecord>,
}

impl Unified for Tasks {
    fn fqn() -> Fqn {
        Fqn::root("@tasks")
    }
}

impl Flow for Tasks {
    type Event = TasksEvent;
    type Action = ();

    fn apply(&mut self, event: Self::Event) {
        match event {
            TasksEvent::Add {
                id,
                parameters,
                deadline,
            } => {}
            TasksEvent::Update { id, deadline } => {}
            TasksEvent::Del { id } => {}
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Role {
    Request,
    Response,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TasksEvent {
    Add {
        id: TaskId,
        deadline: Deadline,
        parameters: TaskParameters,
    },
    Update {
        id: TaskId,
        deadline: Deadline,
    },
    Del {
        id: TaskId,
    },
}
