use rand::Rng;
use crate::state::{Language, SnippetLength};

mod rust;
mod python;
mod javascript;
mod go;
mod c;

pub trait WordBank {
    fn build_snippet(&self, rng: &mut impl Rng, length: SnippetLength) -> String;
}

pub fn generate_snippet(language: Language, length: SnippetLength) -> String {
    let mut rng = rand::thread_rng();
    match language {
        Language::Rust       => rust::RustBank.build_snippet(&mut rng, length),
        Language::Python     => python::PythonBank.build_snippet(&mut rng, length),
        Language::JavaScript => javascript::JavaScriptBank.build_snippet(&mut rng, length),
        Language::Go         => go::GoBank.build_snippet(&mut rng, length),
        Language::C          => c::CBank.build_snippet(&mut rng, length),
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
