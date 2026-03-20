use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    Go,
    C,
}

impl Language {
    pub fn label(&self) -> &'static str {
        match self {
            Language::Rust       => "Rust",
            Language::Python     => "Python",
            Language::JavaScript => "JavaScript",
            Language::Go         => "Go",
            Language::C          => "C",
        }
    }

    pub fn all() -> &'static [Language] {
        &[
            Language::Rust,
            Language::Python,
            Language::JavaScript,
            Language::Go,
            Language::C,
        ]
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
            SnippetLength::Short  => "25",
            SnippetLength::Medium => "50",
            SnippetLength::Long   => "100",
        }
    }

    pub fn word_count(&self) -> usize {
        match self {
            SnippetLength::Short  => 25,
            SnippetLength::Medium => 50,
            SnippetLength::Long   => 100,
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
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            selected_language: Language::Rust,
            snippet_length: SnippetLength::Medium,
        }
    }
}

#[derive(Debug)]
pub struct TypingState {
    pub language: Language,
    pub snippet: Vec<char>,
    pub input: Vec<CharResult>,
    pub cursor: usize,
    pub started_at: Option<Instant>,
    pub finished_at: Option<Instant>,
}

impl TypingState {
    pub fn new(language: Language, snippet: String) -> Self {
        Self {
            language,
            snippet: snippet.chars().collect(),
            input: Vec::new(),
            cursor: 0,
            started_at: None,
            finished_at: None,
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
    pub errors: usize,
    pub chars_typed: usize,
}

pub enum AppState {
    Menu(MenuState),
    Typing(TypingState),
    Results(ResultsData),
}
