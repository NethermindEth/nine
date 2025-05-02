use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use ui9::names::Fqn;
use ui9_dui::flow::{Flow, Unified};
use ui9_dui::publisher::{Publisher, Tracer};
use ui9_dui::subscriber::{Listener, Subscriber};

#[derive(Deref, DerefMut, From, Into)]
pub struct ToolsSub {
    listener: Listener<Tools>,
}

impl Subscriber for Tools {
    type Driver = ToolsSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct ToolsPub {
    tracer: Tracer<Tools>,
}

impl Publisher for Tools {
    type Driver = ToolsPub;
}

impl Unified for Tools {
    fn fqn() -> Fqn {
        Fqn::root("@tools")
    }
}

impl ToolsPub {
    pub fn add_tool(&self, toolkit: String, action: String) {
        let event = ToolsEvent::Add { toolkit, action };
        self.event(event);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tools {
    pub tools_list: BTreeMap<String, BTreeSet<String>>,
}

impl Default for Tools {
    fn default() -> Self {
        Self {
            tools_list: BTreeMap::new(),
        }
    }
}

impl Flow for Tools {
    type Event = ToolsEvent;
    type Action = ();

    fn apply(&mut self, event: Self::Event) {
        use ToolsEvent::*;
        match event {
            Add { toolkit, action } => {
                self.tools_list.entry(toolkit).or_default().insert(action);
            }
            Del { toolkit, action } => {
                self.tools_list.entry(toolkit).or_default().remove(&action);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolsEvent {
    Add { toolkit: String, action: String },
    Del { toolkit: String, action: String },
}
