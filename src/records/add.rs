use crate::records::model::RunRecord;

pub fn add(records: &mut Vec<RunRecord>, record: RunRecord) {
    records.push(record);
    crate::records::save::save(records);
}
