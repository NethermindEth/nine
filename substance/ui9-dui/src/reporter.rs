use crate::tracers::event::{Event, EventData};
use crate::tracers::failure::{Failure, FailureData};
use crate::tracers::job::{Job, JobData, OperationId};
use crate::{Act, Pub};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::core::time::Instant;
use crb::superagent::{AgentBridge, StreamSession};
use futures::Future;
use std::sync::LazyLock;

static LOG_BRIDGE: LazyLock<AgentBridge<Reporter>> = LazyLock::new(|| AgentBridge::new());

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
    pub fn wrap_fn<F, T>(task: &str, func: F) -> Result<T>
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

    pub async fn wrap_fut<F, T>(task: &str, fut: F) -> Result<T>
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

pub struct Reporter {
    job: Pub<Job>,
    event: Pub<Event>,
    failure: Pub<Failure>,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            job: Pub::unified(),
            event: Pub::unified(),
            failure: Pub::unified(),
        }
    }

    /*
    pub fn log(msg: &str) {
        let event = Act {
            action: JobData::Message(msg.into()),
        };
        LOG_BRIDGE.send(event);
    }
    */
}

impl Agent for Reporter {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for Reporter {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        ctx.consume_events(LOG_BRIDGE.events().await?);
        ctx.consume(self.job.actions()?);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Act<Job>> for Reporter {
    async fn handle(&mut self, msg: Act<Job>, _ctx: &mut Context<Self>) -> Result<()> {
        self.job.event(msg.action);
        Ok(())
    }
}

#[async_trait]
impl OnEvent<Act<Event>> for Reporter {
    async fn handle(&mut self, msg: Act<Event>, _ctx: &mut Context<Self>) -> Result<()> {
        self.event.event(msg.action);
        Ok(())
    }
}

#[async_trait]
impl OnEvent<Act<Failure>> for Reporter {
    async fn handle(&mut self, msg: Act<Failure>, _ctx: &mut Context<Self>) -> Result<()> {
        self.failure.event(msg.action);
        Ok(())
    }
}
