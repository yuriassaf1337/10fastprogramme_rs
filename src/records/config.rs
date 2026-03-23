use std::path::PathBuf;

pub fn data_path() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join("10fastprogramme").join("records.json"))
}
