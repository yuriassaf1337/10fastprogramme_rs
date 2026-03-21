use crate::words::available_languages;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub struct Language(pub String);

impl Language {
    pub fn label(&self) -> String {
        // This is retarded. who cares
        match self.0.to_lowercase().as_str() {
            s => {
                let mut c = s.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn all() -> Vec<Language> {
        available_languages().into_iter().map(Language).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SnippetLength {
    Short,  // ~25 words
    Medium, // ~50 words
    Long,   // ~100 words
}

impl SnippetLength {
    pub fn label(&self) -> &'static str {
        match self {
            SnippetLength::Short => "25",
            SnippetLength::Medium => "50",
            SnippetLength::Long => "100",
        }
    }

    pub fn word_count(&self) -> usize {
        match self {
            SnippetLength::Short => 25,
            SnippetLength::Medium => 50,
            SnippetLength::Long => 100,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CharResult {
    Correct,
    Incorrect,
}

#[derive(Debug)]
pub struct MenuState {
    pub selected_language: Language,
    pub snippet_length: SnippetLength,
    pub languages: Vec<Language>,
}

impl Default for MenuState {
    fn default() -> Self {
        let languages = Language::all();
        let selected_language = languages
            .iter()
            .find(|l| l.name() == "rust")
            .cloned()
            .unwrap_or_else(|| {
                languages
                    .first()
                    .cloned()
                    .unwrap_or(Language("rust".into()))
            });

        Self {
            selected_language,
            snippet_length: SnippetLength::Medium,
            languages,
        }
    }
}

#[derive(Debug)]
pub struct TypingState {
    pub language: Language,
    pub snippet_length: SnippetLength,
    pub snippet: Vec<char>,
    pub input: Vec<CharResult>,
    pub cursor: usize,
    pub started_at: Option<Instant>,
    pub finished_at: Option<Instant>,
    pub wpm_history: Vec<f32>,
    pub accuracy_history: Vec<f32>,
}

impl TypingState {
    pub fn new(language: Language, snippet_length: SnippetLength, snippet: String) -> Self {
        Self {
            language,
            snippet_length,
            snippet: snippet.chars().collect(),
            input: Vec::new(),
            cursor: 0,
            started_at: None,
            finished_at: None,
            wpm_history: Vec::new(),
            accuracy_history: Vec::new(),
        }
    }

    pub fn is_complete(&self) -> bool {
        self.cursor >= self.snippet.len()
    }
}

#[derive(Debug, Clone)]
pub struct ResultsData {
    pub wpm: f32,
    pub accuracy: f32,
    pub time_elapsed: f32,
    pub language: Language,
    pub snippet_length: SnippetLength,
    pub errors: usize,
    pub wpm_history: Vec<f32>,
    pub accuracy_history: Vec<f32>,
}

pub enum AppState {
    Menu(MenuState),
    Typing(TypingState),
    Results(ResultsData),
}
