//! The grammar of a `specialformat` template (design D6 of `m2-parser-breadth`; the old compiler reads statements
//! by these templates, `seperateargs`, `study\02` §4.3): `?` is an argument, `{A|B C}` a choice of word sequences
//! (an item may be a punctuation character: `{#|LPrint}`, `{Len =}`), `[…]` optional, any other character literal
//! punctuation. Used by `build.rs` (every template must parse, or the build fails) and by the parser.

/// One element of a template.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    /// `?`: an expression.
    Arg,
    /// A punctuation character: `,`, `(`, `)`, `-`, `=` or `#`.
    Punct(u8),
    /// `{…|…}`: one of these sequences of words and punctuation, tried in order.
    Choice(Vec<Vec<Atom>>),
    /// `[…]`: the items, or nothing.
    Optional(Vec<Item>),
}

/// An element of a choice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Atom {
    /// A word, as written in the template (matched without regard to case).
    Word(String),
    Punct(u8),
}

/// The punctuation a template may hold outside choices.
const PUNCT: &[u8] = b",()-=#";

/// Parses a template; `Err` names what is wrong.
pub fn parse(template: &str) -> Result<Vec<Item>, String> {
    let bytes = template.as_bytes();
    let mut pos = 0;
    let items = seq(bytes, &mut pos, None)?;
    if pos != bytes.len() {
        return Err(format!("unexpected `{}` at {pos} in {template:?}", bytes[pos] as char));
    }
    Ok(items)
}

/// Items up to `close` (or the end).
fn seq(b: &[u8], pos: &mut usize, close: Option<u8>) -> Result<Vec<Item>, String> {
    let mut out = Vec::new();
    while *pos < b.len() {
        let c = b[*pos];
        if Some(c) == close {
            return Ok(out);
        }
        *pos += 1;
        match c {
            b'?' => out.push(Item::Arg),
            b'[' => {
                let inner = seq(b, pos, Some(b']'))?;
                if b.get(*pos) != Some(&b']') {
                    return Err("`[` without `]`".into());
                }
                *pos += 1;
                out.push(Item::Optional(inner));
            }
            b'{' => {
                let end = b[*pos..].iter().position(|&x| x == b'}').ok_or("`{` without `}`")?;
                let body = &b[*pos..*pos + end];
                *pos += end + 1;
                let mut alts = Vec::new();
                for alt in body.split(|&x| x == b'|') {
                    let atoms: Vec<Atom> = alt
                        .split(|&x| x == b' ')
                        .filter(|w| !w.is_empty())
                        .map(|w| match w {
                            [p] if PUNCT.contains(p) => Atom::Punct(*p),
                            // Template words are ASCII.
                            _ => Atom::Word(w.iter().map(|&b| char::from(b)).collect()),
                        })
                        .collect();
                    if atoms.is_empty() {
                        return Err("an empty choice".into());
                    }
                    alts.push(atoms);
                }
                out.push(Item::Choice(alts));
            }
            c if PUNCT.contains(&c) => out.push(Item::Punct(c)),
            c => return Err(format!("unexpected `{}`", c as char)),
        }
    }
    match close {
        Some(c) => Err(format!("missing `{}`", c as char)),
        None => Ok(out),
    }
}

/// Prints a template back (the inverse of [`parse`]).
pub fn print(items: &[Item]) -> String {
    let mut out = String::new();
    for item in items {
        match item {
            Item::Arg => out.push('?'),
            Item::Punct(c) => out.push(*c as char),
            Item::Optional(inner) => {
                out.push('[');
                out.push_str(&print(inner));
                out.push(']');
            }
            Item::Choice(alts) => {
                out.push('{');
                let alts: Vec<String> = alts
                    .iter()
                    .map(|a| {
                        a.iter()
                            .map(|atom| match atom {
                                Atom::Word(w) => w.clone(),
                                Atom::Punct(c) => (*c as char).to_string(),
                            })
                            .collect::<Vec<_>>()
                            .join(" ")
                    })
                    .collect();
                out.push_str(&alts.join("|"));
                out.push('}');
            }
        }
    }
    out
}
