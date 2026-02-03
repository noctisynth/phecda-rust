use serde_json::Value;

#[derive(Default)]
pub enum Level {
    /// Indicates very spammy debug information.
    Debug,
    /// Informational messages.
    #[default]
    Info,
    /// A warning.
    Warning,
    /// An error.
    Error,
    /// Similar to error but indicates a critical event that usually causes a shutdown.
    Fatal,
}

pub struct MetaInfo {}

pub struct EventContent {
    pub message: String,
    pub params: Vec<Value>,
}

pub struct Event {
    pub meta: MetaInfo,
    pub level: Level,
    pub data: EventContent,
}
