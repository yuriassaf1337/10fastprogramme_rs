use crate::records::model::RunRecord;

pub fn personal_best<'a>(
    records: &'a [RunRecord],
    language: &str,
    snippet_length: &str,
) -> Option<&'a RunRecord> {
    records
        .iter()
        .filter(|r| r.completed && r.language == language && r.snippet_length == snippet_length)
        .max_by(|a, b| a.wpm.partial_cmp(&b.wpm).unwrap_or(std::cmp::Ordering::Equal))
}
