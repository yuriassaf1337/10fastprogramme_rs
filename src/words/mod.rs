use rand::Rng;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

pub fn data_dir() -> PathBuf {
    let base = resolve_base_dir();
    let languages_dir = base.join("languages");
    if !languages_dir.exists() {
        fs::create_dir_all(&languages_dir).expect("failed to create app data directory");
        seed_default_languages(&languages_dir);
    }
    languages_dir
}

fn resolve_base_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        dirs::document_dir()
            .expect("could not locate Documents folder")
            .join("10fastprogramme_rs")
    }
    #[cfg(not(target_os = "windows"))]
    {
        dirs::home_dir()
            .expect("could not locate home directory")
            .join("10fastprogramme_rs")
    }
}

fn seed_default_languages(dir: &Path) {
    let defaults: &[(&str, &str)] = &[
        ("rust",       include_str!("rust.json")),
        ("python",     include_str!("python.json")),
        ("javascript", include_str!("javascript.json")),
        ("go",         include_str!("go.json")),
        ("c",          include_str!("c.json")),
    ];

    for (name, content) in defaults {
        let path = dir.join(format!("{}.json", name));
        fs::write(&path, content)
            .unwrap_or_else(|e| eprintln!("warning: could not seed {name}.json: {e}"));
    }
}

#[derive(Debug, Deserialize)]
pub struct LanguageBank {
    pub identifiers: Vec<String>,
    pub templates: Vec<String>,
}

impl LanguageBank {
    pub fn load(name: &str) -> Option<Self> {
        let path = data_dir().join(format!("{}.json", name));
        let text = fs::read_to_string(&path)
            .map_err(|e| eprintln!("warning: could not read {}: {e}", path.display()))
            .ok()?;
        serde_json::from_str(&text)
            .map_err(|e| eprintln!("warning: could not parse {}: {e}", path.display()))
            .ok()
    }

    pub fn build_snippet(&self, rng: &mut impl Rng, word_count: usize) -> String {
        if self.templates.is_empty() || self.identifiers.is_empty() {
            return String::new();
        }

        let id_refs: Vec<&str> = self.identifiers.iter().map(String::as_str).collect();
        let mut words: Vec<String> = Vec::new();
        let mut i = 0usize;

        while words.len() < word_count {
            let template = &self.templates[i % self.templates.len()];
            let filled = fill_template(template, &id_refs, rng);
            for word in filled.split_whitespace() {
                words.push(word.to_string());
            }
            i += 1;
        }

        words.truncate(word_count);
        words.join(" ")
    }
}

pub fn fill_template(template: &str, identifiers: &[&str], rng: &mut impl Rng) -> String {
    let mut result = template.to_string();
    while result.contains("{id}") {
        let replacement = identifiers[rng.gen_range(0..identifiers.len())];
        let pos = result.find("{id}").unwrap();
        result.replace_range(pos..pos + 4, replacement);
    }
    result
}

pub fn available_languages() -> Vec<String> {
    let dir = data_dir();
    let mut names: Vec<String> = fs::read_dir(&dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter_map(|e| {
                    let p = e.path();
                    if p.extension().and_then(|x| x.to_str()) == Some("json") {
                        p.file_stem()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_string())
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    names.sort();
    names
}

pub fn generate_snippet(language_name: &str, word_count: usize) -> String {
    let mut rng = rand::thread_rng();
    match LanguageBank::load(language_name) {
        Some(bank) => bank.build_snippet(&mut rng, word_count),
        None => {
            eprintln!("warning: language '{}' not found", language_name);
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_template_replaces_all_ids() {
        let ids = &["foo", "bar"];
        let mut rng = rand::thread_rng();
        let result = fill_template("{id} = {id};", ids, &mut rng);
        assert!(
            !result.contains("{id}"),
            "all {{id}} placeholders should be replaced"
        );
    }

    #[test]
    fn test_fill_template_no_placeholders() {
        let ids = &["foo"];
        let mut rng = rand::thread_rng();
        let result = fill_template("package main", ids, &mut rng);
        assert_eq!(result, "package main");
    }

    #[test]
    fn test_language_bank_build_snippet_length() {
        let bank = LanguageBank {
            identifiers: vec!["x".into(), "y".into()],
            templates: vec!["{id} = {id};".into(), "let {id} = {id};".into()],
        };
        let mut rng = rand::thread_rng();
        let snippet = bank.build_snippet(&mut rng, 10);
        let count = snippet.split_whitespace().count();
        assert_eq!(count, 10, "snippet should contain exactly 10 tokens");
    }
}
