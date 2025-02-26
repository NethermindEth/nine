use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tools {
    pub tools_list: BTreeMap<String, String>,
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
            Add { id, description } => {
                self.tools_list.insert(id, description);
            }
            Del { id } => {
                self.tools_list.remove(&id);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolsEvent {
    Add { id: String, description: String },
    Del { id: String },
}
