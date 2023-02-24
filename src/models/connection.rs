use std::time::Duration;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ConnectionSettings{
    pub retry_count: i32,
    pub retry_delay: Duration,
}

impl Default for ConnectionSettings {
    fn default() -> Self {
        Self {  
            retry_count: 10,
            retry_delay: Duration::from_secs(5)
        }
    }
}