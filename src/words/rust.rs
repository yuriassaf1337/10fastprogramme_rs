use rand::Rng;
use rand::seq::SliceRandom;
use crate::state::SnippetLength;
use super::{WordBank, fill_template};

pub struct RustBank;

const IDENTIFIERS: &[&str] = &[
    "value", "result", "index", "count", "name", "data", "input",
    "output", "error", "item", "list", "map", "key", "node", "root",
    "left", "right", "depth", "width", "height", "size", "len",
    "buf", "src", "dst", "path", "file", "line", "col", "config",
    "state", "ctx", "handle", "stream", "bytes", "token", "parser",
    "writer", "reader", "entry", "field", "payload", "cache",
];

const TEMPLATES: &[&str] = &[
    "let {id} = {id};",
    "let mut {id} = Vec::new();",
    "let {id}: usize = 0;",
    "fn {id}({id}: &{id}) -> {id} {",
    "fn {id}({id}: usize) -> Option<{id}> {",
    "pub fn {id}(&self) -> &{id} {",
    "pub fn {id}(&mut self, {id}: {id}) {",
    "if let Some({id}) = {id} {",
    "if let Ok({id}) = {id} {",
    "match {id} {",
    "for {id} in {id}.iter() {",
    "for {id} in 0..{id} {",
    "while let Some({id}) = {id}.next() {",
    "{id}.push({id});",
    "{id}.insert({id}, {id});",
    "return Ok({id});",
    "return Err({id});",
    "return Some({id});",
    "println!(\"{}\", {id});",
    "eprintln!(\"{}\", {id});",
    "let {id} = {id}.unwrap();",
    "let {id} = {id}.unwrap_or_default();",
    "let {id} = {id}?;",
    "let {id} = {id}.len();",
    "let {id} = {id}.is_empty();",
    "impl {id} {",
    "impl {id} for {id} {",
    "struct {id} {",
    "enum {id} {",
    "use std::collections::HashMap;",
    "use std::path::PathBuf;",
    "type {id} = Vec<{id}>;",
    "const {id}: usize = 0;",
    "let {id} = {id}.clone();",
    "let {id} = {id}.to_string();",
    "let {id} = {id}.as_str();",
    "let {id} = {id}.parse::<usize>().unwrap();",
    "{id}.iter().map(|{id}| {id}).collect()",
    "{id}.iter().filter(|{id}| {id}).count()",
    "assert_eq!({id}, {id});",
    "#[derive(Debug, Clone)]",
];

impl WordBank for RustBank {
    fn build_snippet(&self, rng: &mut impl Rng, length: SnippetLength) -> String {
        let target = length.word_count();
        let mut lines: Vec<String> = TEMPLATES.to_vec()
            .into_iter()
            .map(|t| t.to_string())
            .collect();
        lines.shuffle(rng);

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
