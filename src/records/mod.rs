pub mod add;
pub mod config;
pub mod load;
pub mod model;
pub mod personal_best;
pub mod recent;
pub mod save;
pub mod utils;
#[cfg(test)]
mod tests;

use model::RunRecord;

pub struct Records {
    entries: Vec<RunRecord>,
}

impl Records {
    pub fn load() -> Self {
        Self {
            entries: load::load(),
        }
    }

    pub fn add(&mut self, record: RunRecord) {
        add::add(&mut self.entries, record);
    }

    pub fn recent(&self, n: usize) -> Vec<&RunRecord> {
        recent::recent(&self.entries, n)
    }

    pub fn personal_best(&self, language: &str, snippet_length: &str) -> Option<&RunRecord> {
        personal_best::personal_best(&self.entries, language, snippet_length)
    }
}
