use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRecord {
    pub language: String,
    pub snippet_length: String,
    pub wpm: f32,
    pub accuracy: f32,
    pub time_elapsed: f32,
    pub errors: usize,
    pub completed: bool,
    pub timestamp: String,
}

impl RunRecord {
    pub fn new(
        language: &str,
        snippet_length: &str,
        wpm: f32,
        accuracy: f32,
        time_elapsed: f32,
        errors: usize,
        completed: bool,
    ) -> Self {
        Self {
            language: language.to_owned(),
            snippet_length: snippet_length.to_owned(),
            wpm,
            accuracy,
            time_elapsed,
            errors,
            completed,
            timestamp: crate::records::utils::now_iso8601(),
        }
    }
}
