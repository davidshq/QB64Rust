//! The extension's block snippets (`vscode\snippets\qb64rust.code-snippets`) compile as inserted: each body, with
//! every placeholder at its default, is a program the front end accepts (spec `editor/language-support`, "Block
//! snippets").

use crate::support::repo;
use qb64rust_driver::frontend;
use serde_json::Value;

/// A snippet body with each placeholder as VS Code inserts it before anything is typed: `${1:name}` is `name`,
/// `${2}`, `$1` and `$0` are empty.
fn expand(body: &str) -> String {
    let mut out = String::new();
    let mut rest = body;
    while let Some(at) = rest.find('$') {
        out.push_str(&rest[..at]);
        rest = &rest[at + 1..];
        if let Some(inner) = rest.strip_prefix('{') {
            let end = inner.find('}').expect("a placeholder ends with }");
            if let Some((_, default)) = inner[..end].split_once(':') {
                out.push_str(default);
            }
            rest = &inner[end + 1..];
        } else {
            rest = rest.trim_start_matches(|c: char| c.is_ascii_digit());
        }
    }
    out.push_str(rest);
    out
}

#[test]
fn placeholders_take_their_defaults() {
    assert_eq!(expand("SUB ${1:MySub} (${2})"), "SUB MySub ()");
    assert_eq!(expand("\t$0"), "\t");
}

#[test]
fn each_block_snippet_compiles() {
    let path = repo().join("vscode").join("snippets").join("qb64rust.code-snippets");
    let snippets: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    let snippets = snippets.as_object().unwrap();
    assert!(!snippets.is_empty());
    for (name, snippet) in snippets {
        let lines: Vec<String> = snippet["body"]
            .as_array()
            .unwrap()
            .iter()
            .map(|l| expand(l.as_str().unwrap()))
            .collect();
        let source = lines.join("\r\n") + "\r\n";
        let fe = frontend("snippet.bas", source.clone().into_bytes());
        assert!(
            !fe.has_errors(),
            "snippet {name:?} does not compile:\n{source}\n{:?}",
            fe.diagnostics
        );
    }
}
