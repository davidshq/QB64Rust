//! SUB calls and built-in statements: the layout the SUB-call handler builds from `seperateargs`' layout lists
//! (`qb64pe.bas` 11560-12415, `seperateargs` 26211).
//!
//! The statement starts with its name and `sp` (with `CALL`: `Call name(`). Before each argument stand the template's
//! items since the argument before it: punctuation, words that are no choice (`{As}` of `OPEN`), and a chosen
//! option (`{Step}`, `{B|BF}`), which is an argument of its own with no expression. Each item takes its spacing from
//! the end of the layout so far and from what it is (`customlaychar`); a word after a name or an argument gets a space
//! before it; an argument after a word gets a space before it (`convertspacing`). The last separator is dropped.

use crate::apply::{SP, SP2};
use crate::fmt::Fmt;
use crate::lay::{Lay, scase};
use qb64rust_syntax::SyntaxKind::*;
use qb64rust_syntax::tree::{Element, Node};

/// QB64pe's `alphanumeric` table: digits, letters and `_`.
fn alnum(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

/// One item of a layout list, `followed`: the option marker follows it (a chosen option).
struct Item {
    text: Vec<u8>,
    option: bool,
}

/// The state the SUB-call handler carries from item to item.
#[derive(Default)]
struct Args {
    /// `addedlayout`: an argument's layout was the last thing added.
    added_layout: bool,
    /// `convertspacing`: the last item of the list before the argument starts with a letter, digit or `_`.
    convert: bool,
    /// The list is the last argument's of `TIMER` or `KEY`: a space before each item but `)`.
    last_arg: bool,
}

impl Fmt<'_> {
    /// `name args` of a built-in statement read by its template.
    pub(crate) fn builtin_stmt(&self, n: Node<'_>) -> Option<Lay> {
        let mut elems = self.elems(n);
        // `CALL name(args)` (`usecall = 1`): the parentheses are the call's, not the template's.
        let call = matches!(elems.first(), Some(Element::Token(t)) if self.is_word(*t, "CALL"));
        // `CALL name` with no parentheses is `usecall = 2`: `Call name args`.
        let mut parens = false;
        if call {
            elems.remove(0);
            if matches!(elems.get(1), Some(Element::Token(t)) if t.kind == LParen)
                && matches!(elems.last(), Some(Element::Token(t)) if t.kind == RParen)
            {
                elems.remove(1);
                elems.pop();
                parens = true;
            }
        }
        let Some(Element::Token(name)) = elems.first().copied() else {
            return None;
        };
        let written = self.tok(name);
        let entries: Vec<_> = qb64rust_builtins::find_any(&written[..crate::fmt::name_len(written)])
            .filter(|b| b.kind == qb64rust_builtins::Kind::Sub)
            .collect();
        let first = entries.first()?;
        let mut l = Lay::new();
        if call {
            l.text(&self.kw("CALL")).sp();
        }
        l.text(&scase(first.name.as_bytes(), self.style()));
        l.text(first.musthave.unwrap_or("").as_bytes());
        // `sub_timer` and `sub_key` take no space after the name (`qb64pe.bas` 11640), and before each item of the
        // last argument's list but `)`, one.
        let timer_key = !call
            && entries
                .iter()
                .any(|b| b.callname == "sub_timer" || b.callname == "sub_key");
        if parens {
            l.sp2().text(b"(").sp2();
        } else if timer_key {
            l.sp2();
        } else {
            l.sp();
        }
        // An argument of a slot that takes a numeric array (`-3`, graphics `GET`/`PUT`): a bare array name gets `()`.
        let array_slot = entries.iter().any(|b| {
            b.arg_types
                .iter()
                .any(|t| t.starts_with("offset+size (largest safe block, restricted"))
        });
        let options = template_options(entries.iter().filter_map(|b| b.specialformat));
        let mut pending: Vec<Item> = Vec::new();
        let mut st = Args::default();
        let mut i = 1;
        while i < elems.len() {
            match elems[i] {
                Element::Token(t) => {
                    pending.push(Item {
                        text: self.tok(t).to_vec(),
                        option: false,
                    });
                    i += 1;
                }
                Element::Node(c) if c.kind() == FormArg => {
                    self.items(&mut l, &pending, &mut st);
                    pending.clear();
                    let mut arg = c.child_nodes().next().map(|e| self.expr(e)).unwrap_or_default();
                    if array_slot && c.child_nodes().next().is_some_and(|e| self.numeric_array(e)) {
                        arg.text(b"(").sp2().text(b")");
                    }
                    self.argument(&mut l, &arg, &mut st);
                    i += 1;
                }
                Element::Node(c) if c.kind() == FormWord => {
                    // The longest option of the templates that the words from here spell.
                    let ahead: Vec<Vec<u8>> = elems[i..]
                        .iter()
                        .map_while(|e| match e {
                            Element::Node(w) if w.kind() == FormWord => Some(self.text(w.span()).to_ascii_uppercase()),
                            Element::Token(t) if t.kind == Eq => Some(b"=".to_vec()),
                            _ => None,
                        })
                        .collect();
                    let (spelling, words, choice) = options
                        .iter()
                        .filter(|o| {
                            o.words.len() <= ahead.len()
                                && o.words.iter().zip(&ahead).all(|(w, a)| w.as_bytes() == a.as_slice())
                        })
                        .max_by_key(|o| o.words.len())
                        .map_or_else(
                            || (self.text(c.span()).to_vec(), 1, true),
                            |o| (o.spelling.as_bytes().to_vec(), o.words.len(), o.choice),
                        );
                    pending.push(Item {
                        text: scase(&spelling, self.style()),
                        option: choice,
                    });
                    if choice {
                        // A chosen option is an argument: its list ends with it, and it has no expression.
                        st.last_arg = timer_key && i + words >= elems.len();
                        self.items(&mut l, &pending, &mut st);
                        st.last_arg = false;
                        pending.clear();
                    }
                    i += words;
                }
                Element::Node(c) => {
                    pending.push(Item {
                        text: self.trimmed(c),
                        option: false,
                    });
                    i += 1;
                }
            }
        }
        self.items(&mut l, &pending, &mut st);
        if matches!(l.last(), Some(SP | SP2)) {
            l.pop();
        }
        if parens {
            l.sp2().text(b")");
        }
        Some(l)
    }

    /// Whether an argument is a bare name of an array that holds no strings.
    fn numeric_array(&self, e: Node<'_>) -> bool {
        if e.kind() != NameRef {
            return false;
        }
        let Some(t) = e.first_token() else {
            return false;
        };
        let Some(id) = self.program.symbols.at(t.span.file, t.span.start) else {
            return false;
        };
        match self.program.symbols.get(id).kind {
            qb64rust_sema::SymbolKind::Var(v) => {
                let var = self.program.var(v);
                var.is_array() && !matches!(var.ty, qb64rust_sema::Ty::Str | qb64rust_sema::Ty::FixedStr(_))
            }
            _ => false,
        }
    }

    /// A SUB call: `name args`, or `Call name(args)` (`qb64pe.bas` 11603-11612).
    pub(crate) fn call_stmt(&self, n: Node<'_>) -> Option<Lay> {
        let elems = self.elems(n);
        let mut l = Lay::new();
        let mut call = false;
        let mut st = Args::default();
        // `CALL name(args)`: the arguments stand in parentheses.
        let in_parens = elems.iter().any(
            |e| matches!(e, Element::Node(c) if c.kind() == ArgList && c.child_tokens().any(|t| t.kind == LParen)),
        );
        for e in &elems {
            match *e {
                Element::Token(t) if self.is_word(t, "CALL") => {
                    call = true;
                    l.text(&self.kw("CALL")).sp();
                }
                Element::Node(c) if c.kind() == NameRef => {
                    for t in c.child_tokens() {
                        // `CALL ABSOLUTE(…)` with its parentheses is the old compiler's own statement, spelled
                        // from its layout (`qb64pe.bas` 10622, 10688); without them `ABSOLUTE` is a SUB's name.
                        if call && in_parens && self.is_word(t, "ABSOLUTE") {
                            l.text(&self.kw("ABSOLUTE"));
                        } else {
                            l.text(&self.name(t));
                        }
                    }
                    l.sp();
                }
                Element::Node(c) if c.kind() == ArgList => {
                    if call && in_parens {
                        l.set_last(SP2);
                        l.text(b"(").sp2();
                    }
                    let mut pending = Vec::new();
                    for a in self.elems(c) {
                        match a {
                            Element::Token(t) if t.kind == Comma => pending.push(Item {
                                text: b",".to_vec(),
                                option: false,
                            }),
                            Element::Token(_) => {}
                            Element::Node(x) => {
                                self.items(&mut l, &pending, &mut st);
                                pending.clear();
                                let arg = self.expr(x);
                                self.argument(&mut l, &arg, &mut st);
                            }
                        }
                    }
                    self.items(&mut l, &pending, &mut st);
                }
                Element::Node(_) => return None,
                Element::Token(_) => {}
            }
        }
        if matches!(l.last(), Some(SP | SP2)) {
            l.pop();
        }
        if call && in_parens {
            l.sp2().text(b")");
        }
        Some(l)
    }

    /// The items of one layout list (the `DO WHILE LEN(x$)` loop of the SUB-call handler).
    fn items(&self, l: &mut Lay, items: &[Item], st: &mut Args) {
        st.convert = false;
        for it in items {
            let (mut s, mut an) = (0u8, false);
            match l.last() {
                Some(SP) => s = 1,
                Some(SP2) => {
                    s = 2;
                    an = l.0.len() >= 2 && alnum(l.0[l.0.len() - 2]);
                }
                Some(c) => an = alnum(c),
                None => {}
            }
            let s1 = s;
            let mut x2 = it.text.clone();
            let word = x2.first().is_some_and(|&c| alnum(c));
            st.convert = word;
            if (an || st.added_layout) && word {
                s = 1;
                x2.push(SP2);
            } else if x2 == b"=" {
                s = 1;
                x2.push(SP);
            } else if x2 == b"#" {
                s = 1;
                x2.push(SP2);
            } else if x2 == b"," {
                x2.push(SP);
            } else if it.option {
                // `IF x2$ = "STEP"`: the spelling is compared as written by `SCase$`, so in CamelCase `Step` takes `sp`.
                x2.push(if x2 == b"STEP" { SP2 } else { SP });
            } else {
                x2.push(SP2);
            }
            if s == 0 {
                s = 2;
            }
            if s != s1 {
                if s1 != 0 {
                    l.pop();
                }
                l.0.push(if s == 1 { SP } else { SP2 });
            }
            if st.last_arg && it.text != b")" {
                l.set_last(SP);
            }
            l.text(&x2);
            st.added_layout = false;
        }
    }

    /// An argument's layout after its list: a space before it when the list ended with a word.
    fn argument(&self, l: &mut Lay, arg: &Lay, st: &mut Args) {
        if st.convert {
            l.set_last(SP);
        }
        l.append(arg);
        st.added_layout = true;
    }
}

/// An option of a template: its spelling, its words in capitals, and whether it is a choice (an argument) rather than
/// a word with no choice (`{As}` at the top level).
struct TemplateOption {
    spelling: String,
    words: Vec<String>,
    choice: bool,
}

/// The options of the `{...}` groups of templates.
fn template_options<'a>(templates: impl Iterator<Item = &'a str>) -> Vec<TemplateOption> {
    let mut out = Vec::new();
    for t in templates {
        let mut level = 0;
        let mut rest = t;
        while let Some(c) = rest.chars().next() {
            match c {
                '[' => level += 1,
                ']' => level -= 1,
                '{' => {
                    let end = rest.find('}').unwrap_or(rest.len());
                    let group: Vec<&str> = rest[1..end].split('|').collect();
                    // A group of one option is never passed (`DontPass`, `seperateargs` 26455): only a choice is.
                    let _ = level;
                    let choice = group.len() > 1;
                    for o in &group {
                        out.push(TemplateOption {
                            spelling: (*o).to_string(),
                            words: o.split(' ').map(str::to_ascii_uppercase).collect(),
                            choice,
                        });
                    }
                    rest = &rest[end..];
                }
                _ => {}
            }
            rest = &rest[c.len_utf8()..];
        }
    }
    out
}
