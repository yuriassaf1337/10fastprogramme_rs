use crate::records::model::RunRecord;

pub fn recent(records: &[RunRecord], n: usize) -> Vec<&RunRecord> {
    records.iter().rev().take(n).collect()
}
