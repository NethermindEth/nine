use crate::reporter::LOG_BRIDGE;
use crate::tracers::event::{Event, EventData};
use crate::tracers::failure::{Failure, FailureData};
use crate::tracers::job::{Job, JobData, OperationId};
use crate::Act;
use anyhow::Result;
use chrono::{DateTime, Local};
use crb::core::time::Instant;
use futures::Future;

pub struct Operation {
    timestamp: DateTime<Local>,
    started: Instant,
    id: OperationId,
    /// If taken (empty) the task is considered as completed
    task: Option<String>,
}

impl Drop for Operation {
    fn drop(&mut self) {
        if let Some(message) = self.task.take() {
            self.close();
            // Operations must be explicitly completed
            self.send_failure(&format!("Failed: {message}"));
        }
    }
}

impl Operation {
    pub fn scoped_fn<F, T>(task: &str, func: F) -> Result<T>
    where
        F: Fn() -> Result<T>,
    {
        let op = Operation::start(task);
        match func() {
            Ok(value) => {
                op.end(task);
                Ok(value)
            }
            Err(err) => {
                op.failed(&err.to_string());
                Err(err)
            }
        }
    }

    pub async fn scoped_fut<F, T>(task: &str, fut: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        let op = Operation::start(task);
        match fut.await {
            Ok(value) => {
                op.end(task);
                Ok(value)
            }
            Err(err) => {
                op.failed(&err.to_string());
                Err(err)
            }
        }
    }

    pub fn start(task: &str) -> Self {
        let id = OperationId::new();
        let mut this = Self {
            timestamp: Local::now(),
            started: Instant::now(),
            id,
            task: Some(task.into()),
        };
        this.act_job(JobData::Begin {
            id,
            task: task.into(),
        });
        this
    }

    pub fn failed(mut self, message: &str) {
        self.close();

        self.send_failure(message);
    }

    pub fn end(mut self, message: &str) {
        self.close();

        let duration = self.started.elapsed();
        self.act_event(EventData {
            message: message.into(),
            duration,
        });
    }

    fn send_failure(&mut self, reason: &str) {
        let data = FailureData {
            reason: reason.into(),
        };
        self.act_failure(data);
    }

    fn close(&mut self) {
        self.task.take();
        self.act_job(JobData::End { id: self.id });
    }

    fn act_job(&mut self, action: JobData) {
        let event = Act::<Job> { action };
        LOG_BRIDGE.event(event);
    }

    fn act_event(&mut self, action: EventData) {
        let event = Act::<Event> { action };
        LOG_BRIDGE.event(event);
    }

    fn act_failure(&mut self, action: FailureData) {
        let event = Act::<Failure> { action };
        LOG_BRIDGE.event(event);
    }
}
