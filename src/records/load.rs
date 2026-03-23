use crate::records::model::RunRecord;
use std::fs;

pub fn load() -> Vec<RunRecord> {
    let Some(path) = crate::records::config::data_path() else {
        return Vec::new();
    };
    let Ok(bytes) = fs::read(&path) else {
        return Vec::new();
    };
    serde_json::from_slice(&bytes).unwrap_or_default()
}
