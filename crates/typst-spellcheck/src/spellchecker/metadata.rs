use std::time::Duration;

pub struct Metadata {
    pub word_count: usize,
    pub paragraph_count: usize,
    pub languagetool_request_time: Duration,
}
