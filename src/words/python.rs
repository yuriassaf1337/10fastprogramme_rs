use rand::Rng;
use crate::state::SnippetLength;
use super::{WordBank, fill_template};

pub struct PythonBank;

const IDENTIFIERS: &[&str] = &[
    "value", "result", "index", "count", "name", "data", "input",
    "output", "error", "item", "items", "mapping", "key", "node",
    "root", "left", "right", "depth", "width", "height", "size",
    "buf", "src", "dst", "path", "file", "line", "col", "config",
    "state", "ctx", "handle", "stream", "token", "parser", "writer",
    "reader", "entry", "field", "payload", "cache", "args", "kwargs",
    "obj", "cls", "self", "response", "request", "message", "text",
];

const TEMPLATES: &[&str] = &[
    "{id} = {id}",
    "{id} = []",
    "{id} = {}",
    "{id} = None",
    "{id} = {id}()",
    "def {id}({id}, {id}):",
    "def {id}(self):",
    "def {id}(self, {id}):",
    "def {id}(self, {id}) -> {id}:",
    "class {id}:",
    "class {id}({id}):",
    "if {id} is None:",
    "if {id} == {id}:",
    "if {id} in {id}:",
    "if not {id}:",
    "for {id} in {id}:",
    "for {id} in range({id}):",
    "for {id}, {id} in enumerate({id}):",
    "while {id} > 0:",
    "return {id}",
    "return None",
    "raise ValueError({id})",
    "raise {id}({id})",
    "{id}.append({id})",
    "{id}.extend({id})",
    "{id}[{id}] = {id}",
    "print({id})",
    "print(f\"{{{id}}}\")",
    "with open({id}) as {id}:",
    "import {id}",
    "from {id} import {id}",
    "{id} = [{id} for {id} in {id}]",
    "{id} = {id}.get({id}, None)",
    "assert {id} == {id}",
    "assert {id} is not None",
    "super().__init__()",
    "self.{id} = {id}",
    "{id} = len({id})",
    "{id} = str({id})",
    "{id} = int({id})",
];

impl WordBank for PythonBank {
    fn build_snippet(&self, rng: &mut impl Rng, length: SnippetLength) -> String {
        let target = length.word_count();
        let mut words: Vec<String> = Vec::new();
        let mut i = 0;

        while words.len() < target {
            let template = &TEMPLATES[i % TEMPLATES.len()];
            let filled = fill_template(template, IDENTIFIERS, rng);
            for word in filled.split_whitespace() {
                words.push(word.to_string());
            }
            i += 1;
        }

        words.truncate(target);
        words.join(" ")
    }
}
