//! `qb64rust --report unsupported`: what the programs of an input set still need (`study\29` §4).
//!
//! Every program goes through the front end with the error cap lifted; each message marked "not supported yet" is
//! counted over the set. The report is for choosing what comes in next; it is no test and has no expected output.

use crate::{FileLoader, frontend_with};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// The programs under `inputs`: a file as given; of a folder every `.bas` file below it (any letter case), sorted,
/// each program once however often the inputs reach it.
/// A program with an `.err` file beside it is left out and counted in the second value: the old compiler rejects
/// it, so what it uses is no demand.
pub fn programs(inputs: &[PathBuf]) -> Result<(Vec<PathBuf>, usize), String> {
    let mut found = Vec::new();
    for input in inputs {
        if input.is_dir() {
            walk(input, &mut found)?;
        } else if input.is_file() {
            found.push(input.clone());
        } else {
            return Err(format!("{} is neither a file nor a folder", input.display()));
        }
    }
    found.sort();
    // A program reached under two spellings (`a\p.bas` and `.\a\p.bas`) is one program; the first spelling stays,
    // so the report names it as it was given.
    let mut seen = BTreeSet::new();
    let mut kept = Vec::new();
    for p in found {
        let key = std::path::absolute(&p).map_err(|e| format!("cannot resolve {}: {e}", p.display()))?;
        if seen.insert(key) {
            kept.push(p);
        }
    }
    let mut found = kept;
    let all = found.len();
    found.retain(|p| !p.with_extension("err").is_file());
    let rejected = all - found.len();
    Ok((found, rejected))
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("cannot read {}: {e}", dir.display()))?;
    for entry in entries {
        let p = entry.map_err(|e| format!("cannot read {}: {e}", dir.display()))?.path();
        if p.is_dir() {
            walk(&p, out)?;
        } else if p.extension().is_some_and(|e| e.eq_ignore_ascii_case("bas")) {
            out.push(p);
        }
    }
    Ok(())
}

/// The errors of one program: the message of each one marked "not supported yet", and how many are not marked.
pub struct Outcome {
    pub unsupported: Vec<String>,
    pub real: usize,
}

/// One program through the front end; included files as in a compile run, with `include_root` as the compiler
/// root.
pub fn outcome(path: &Path, include_root: &Path) -> Result<Outcome, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    if bytes.len() > qb64rust_base::MAX_SOURCE_LEN {
        return Err(format!("{} is too large", path.display()));
    }
    let name = path.to_string_lossy().to_string();
    let mut fe = frontend_with(&name, bytes, |file| Box::new(FileLoader::new(include_root, file, path)));
    crate::check_exe_icon(&mut fe, path);
    crate::check_embedded_files(&mut fe, path);
    let unsupported = fe
        .diagnostics
        .errors()
        .filter(|d| d.unsupported)
        .map(|d| d.message.clone())
        .collect();
    Ok(Outcome {
        unsupported,
        real: fe.diagnostics.error_count() - fe.diagnostics.unsupported_count(),
    })
}

/// One line of the table: a message, the programs it occurs in, the programs where it is the only message (and
/// no other error stands in the way: bringing it in makes them compile), and its uses over the set.
#[derive(Debug, PartialEq, Eq)]
pub struct Row {
    pub message: String,
    pub programs: usize,
    pub alone: usize,
    pub uses: usize,
}

/// The table for a set of outcomes, the most wanted first: by programs, then by programs it alone holds back, then
/// by uses, then by message.
pub fn tabulate(outcomes: &[Outcome]) -> Vec<Row> {
    let mut rows: BTreeMap<&str, Row> = BTreeMap::new();
    for o in outcomes {
        let distinct: BTreeSet<&str> = o.unsupported.iter().map(String::as_str).collect();
        for &m in &distinct {
            let row = rows.entry(m).or_insert_with(|| Row {
                message: m.to_string(),
                programs: 0,
                alone: 0,
                uses: 0,
            });
            row.programs += 1;
            if distinct.len() == 1 && o.real == 0 {
                row.alone += 1;
            }
        }
        for m in &o.unsupported {
            if let Some(row) = rows.get_mut(m.as_str()) {
                row.uses += 1;
            }
        }
    }
    let mut rows: Vec<Row> = rows.into_values().collect();
    rows.sort_by(|a, b| {
        (b.programs, b.alone, b.uses)
            .cmp(&(a.programs, a.alone, a.uses))
            .then_with(|| a.message.cmp(&b.message))
    });
    rows
}

/// The report as text: a summary line, then the table.
pub fn render(outcomes: &[Outcome], rejected: usize) -> String {
    let clean = outcomes
        .iter()
        .filter(|o| o.unsupported.is_empty() && o.real == 0)
        .count();
    let real = outcomes.iter().filter(|o| o.real > 0).count();
    let marked = outcomes.len() - clean - real;
    let n = outcomes.len();
    let mut out = format!(
        "{n} program{}: {clean} without errors, {marked} with only \"not supported yet\" errors, {real} with other \
         errors\n",
        if n == 1 { "" } else { "s" }
    );
    if rejected > 0 {
        out.push_str(&format!(
            "{rejected} left out: an .err file beside them (the old compiler rejects them)\n"
        ));
    }
    let rows = tabulate(outcomes);
    if !rows.is_empty() {
        out.push_str("programs  alone   uses  not supported yet\n");
        for r in rows {
            out.push_str(&format!(
                "{:>8} {:>6} {:>6}  {}\n",
                r.programs, r.alone, r.uses, r.message
            ));
        }
    }
    out
}

/// The whole report for `inputs`.
pub fn unsupported(inputs: &[PathBuf], include_root: &Path) -> Result<String, String> {
    let (programs, rejected) = programs(inputs)?;
    let outcomes = programs
        .iter()
        .map(|p| outcome(p, include_root))
        .collect::<Result<Vec<_>, _>>()?;
    let mut out = render(&outcomes, rejected);
    // A program with an error that is not marked is no demand to count on: name it, so that the cause (an included
    // file not found under this compiler root, say) can be looked at.
    let other: Vec<_> = programs.iter().zip(&outcomes).filter(|(_, o)| o.real > 0).collect();
    if !other.is_empty() {
        out.push_str("with other errors:\n");
        for (p, o) in other {
            out.push_str(&format!("{:>8}  {}\n", o.real, p.display()));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn o(unsupported: &[&str], real: usize) -> Outcome {
        Outcome {
            unsupported: unsupported.iter().map(|m| m.to_string()).collect(),
            real,
        }
    }

    #[test]
    fn counts_programs_alone_and_uses() {
        let outcomes = [
            o(&["`COLOR`", "`COLOR`", "`LOCATE`"], 0),
            o(&["`COLOR`", "`COLOR`", "`COLOR`"], 0),
            // The only message, but a real error stands in the way too.
            o(&["`LOCATE`"], 1),
            o(&[], 0),
        ];
        let row = |message: &str, programs, alone, uses| Row {
            message: message.to_string(),
            programs,
            alone,
            uses,
        };
        assert_eq!(tabulate(&outcomes), [row("`COLOR`", 2, 1, 5), row("`LOCATE`", 2, 0, 2)]);
        assert_eq!(
            render(&outcomes, 2),
            "4 programs: 1 without errors, 2 with only \"not supported yet\" errors, 1 with other errors\n\
             2 left out: an .err file beside them (the old compiler rejects them)\n\
             programs  alone   uses  not supported yet\n       \
             2      1      5  `COLOR`\n       \
             2      0      2  `LOCATE`\n"
        );
    }

    #[test]
    fn ties_are_ordered_by_message() {
        let outcomes = [o(&["b"], 0), o(&["a"], 0)];
        let messages: Vec<_> = tabulate(&outcomes).into_iter().map(|r| r.message).collect();
        assert_eq!(messages, ["a", "b"]);
        assert_eq!(
            render(&[o(&[], 0)], 0),
            "1 program: 1 without errors, 0 with only \"not supported yet\" errors, 0 with other errors\n"
        );
    }
}
