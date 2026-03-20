use rand::Rng;
use crate::state::SnippetLength;
use super::{WordBank, fill_template};

pub struct CBank;

const IDENTIFIERS: &[&str] = &[
    "value", "result", "index", "count", "name", "data", "input",
    "output", "error", "item", "list", "map", "key", "node", "root",
    "left", "right", "depth", "width", "height", "size", "len",
    "buf", "src", "dst", "path", "file", "line", "col", "ptr",
    "tmp", "ret", "n", "i", "j", "k", "p", "q", "s", "c",
    "fd", "fp", "str", "num", "val", "res", "ctx", "handle",
];

const TEMPLATES: &[&str] = &[
    "int {id} = 0;",
    "int {id} = {id};",
    "char *{id} = NULL;",
    "char {id}[256];",
    "void *{id} = NULL;",
    "size_t {id} = 0;",
    "int {id}({id} *{id}, size_t {id});",
    "void {id}({id} *{id});",
    "static int {id}({id} *{id}, int {id});",
    "struct {id} {",
    "typedef struct {id} {id};",
    "if ({id} == NULL) {",
    "if ({id} != NULL) {",
    "if ({id} < 0) {",
    "if ({id} == 0) {",
    "if ({id} >= {id}) {",
    "for (int {id} = 0; {id} < {id}; {id}++) {",
    "for ({id} = {id}; {id} != NULL; {id} = {id}->{id}) {",
    "while ({id} != NULL) {",
    "while ({id}-- > 0) {",
    "switch ({id}) {",
    "case {id}:",
    "return {id};",
    "return NULL;",
    "return -1;",
    "printf(\"%d\\n\", {id});",
    "fprintf(stderr, \"%s\\n\", {id});",
    "malloc(sizeof({id}))",
    "free({id});",
    "{id} = ({id} *) malloc(sizeof({id}));",
    "memset({id}, 0, sizeof({id}));",
    "memcpy({id}, {id}, {id});",
    "strlen({id})",
    "strcmp({id}, {id})",
    "strcpy({id}, {id});",
    "#include <stdio.h>",
    "#include <stdlib.h>",
    "#include <string.h>",
    "#define {id} 0",
    "{id}->{id} = {id};",
    "{id}[{id}] = {id};",
];

impl WordBank for CBank {
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
