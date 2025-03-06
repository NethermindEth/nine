mod chat;
pub mod dashboard;
mod events;
mod jobs;
mod peers;

pub use chat::ChatWidget as Chat;
pub use events::EventsWidget as Events;
pub use jobs::JobsWidget as Jobs;
pub use peers::PeersWidget as Peers;
