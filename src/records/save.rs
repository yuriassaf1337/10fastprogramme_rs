use crate::records::model::RunRecord;
use std::fs;

pub fn save(records: &[RunRecord]) {
    let Some(path) = crate::records::config::data_path() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let Ok(json) = serde_json::to_string_pretty(records) else {
        return;
    };
    let _ = fs::write(&path, json);
}
