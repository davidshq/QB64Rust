//! Expressions: the layout `fixoperationorder$` builds (`qb64pe.bas` 23502, parts H and I).
//!
//! - Elements are joined by `sp`; an operator of more than one character goes through `SCase2$` (`Mod`, `And`).
//! - A negation is glued to what follows it (`-x`, `-(x)`), as part H appends it without a separator.
//! - `(` takes no space before it unless an operator stands there (part I, "spacing just before (", so `f(1)` and
//!   `2 * (1)`), none after it, none before `)` or `,`, and one after `,`.
//! - A `.` between a value and its member joins with `sp2` on both sides.
//! - A number is written as `lineformat$` keeps it: a plain integer without its leading zeros, any other in capitals
//!   as typed (`1.5E3`, `&HFF`, `3#`); a string as it is, its closing quote added when the line ends without one.
//! - Names as the symbol table spells them ([`Fmt::name`]).

use crate::fmt::Fmt;
use crate::lay::Lay;
use qb64rust_syntax::SyntaxKind::*;
use qb64rust_syntax::tree::{Element, Node, Tok};
use std::collections::HashSet;

impl Fmt<'_> {
    /// The layout of an expression node, a whole expression as a handler passes it to `fixoperationorder$`.
    pub fn expr(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        let drops = self.sign_drops(n);
        self.expr_into(n, &mut l, &drops);
        l
    }

    /// The signs part B of `fixoperationorder$` takes out of a whole expression, by the offset of their token: `+ +`
    /// is `+` and `- +` is `-` (the second sign goes), and an operator followed by `- -` loses the pair.
    fn sign_drops(&self, n: Node<'_>) -> HashSet<u32> {
        let mut els: Vec<(Vec<u8>, u32)> = n
            .tokens()
            .into_iter()
            .filter(|t| !t.kind.is_trivia() && t.kind != Newline)
            .map(|t| (self.tok(t).to_ascii_uppercase(), t.span.start))
            .collect();
        let mut drops = HashSet::new();
        let is = |e: &(Vec<u8>, u32), w: &[u8]| e.0 == w;
        // Rules 1 and 2.
        for (first, second) in [(&b"+"[..], &b"+"[..]), (b"-", b"+")] {
            while let Some(i) =
                (0..els.len().saturating_sub(1)).find(|&i| is(&els[i], first) && is(&els[i + 1], second))
            {
                drops.insert(els[i + 1].1);
                els.remove(i + 1);
            }
        }
        // Rule 3.
        while let Some(i) = (0..els.len().saturating_sub(2))
            .find(|&i| is_operator(&els[i].0) && is(&els[i + 1], b"-") && is(&els[i + 2], b"-"))
        {
            drops.insert(els[i + 1].1);
            drops.insert(els[i + 2].1);
            els.drain(i + 1..i + 3);
        }
        drops
    }

    fn expr_into(&self, n: Node<'_>, l: &mut Lay, drops: &HashSet<u32>) {
        match n.kind() {
            Literal => {
                if let Some(t) = n.first_token() {
                    l.text(&self.literal(t));
                }
            }
            NameRef => {
                for t in n.child_tokens() {
                    l.text(&self.name(t));
                }
            }
            CallExpr => {
                for c in n.child_nodes() {
                    match c.kind() {
                        ArgList => self.args_into(c, l, drops),
                        _ => self.expr_into(c, l, drops),
                    }
                }
            }
            ParenExpr => {
                l.text(b"(").sp2();
                for c in n.child_nodes() {
                    self.expr_into(c, l, drops);
                }
                l.sp2().text(b")");
            }
            PrefixExpr => {
                for c in n.children() {
                    match c {
                        Element::Token(t) if t.kind.is_trivia() || drops.contains(&t.span.start) => {}
                        Element::Token(t) => self.unary(t, l),
                        Element::Node(c) => self.expr_into(c, l, drops),
                    }
                }
            }
            BinExpr => {
                // The left operand, the operator, then any signs before the right operand.
                let mut binary = false;
                for c in n.children() {
                    match c {
                        Element::Token(t) if t.kind.is_trivia() || drops.contains(&t.span.start) => {}
                        Element::Token(t) if !binary => {
                            binary = true;
                            l.sp().text(&self.operator(t)).sp();
                        }
                        Element::Token(t) => self.unary(t, l),
                        Element::Node(c) => self.expr_into(c, l, drops),
                    }
                }
            }
            FieldExpr => {
                for c in n.children() {
                    match c {
                        Element::Token(t) if t.kind == Dot => {
                            l.sp2().text(b".").sp2();
                        }
                        Element::Token(t) if t.kind == Ident => {
                            l.text(&self.name(t));
                        }
                        Element::Token(_) => {}
                        Element::Node(c) if c.kind() == ArgList => self.args_into(c, l, drops),
                        Element::Node(c) => self.expr_into(c, l, drops),
                    }
                }
            }
            TypeArg => {
                let words: Vec<&[u8]> = n.child_tokens().map(|t| self.tok(t)).collect();
                l.text(&self.kw2(&words.join(&b'\r')));
            }
            _ => {
                l.text(&self.trimmed(n));
            }
        }
    }

    /// A sign before an operand: a negation glued to it (part C makes it `CHR$(241)` or part of a number), a `+`
    /// that stays an operator with `sp` after it, `NOT` through `SCase2$`.
    fn unary(&self, t: Tok, l: &mut Lay) {
        match t.kind {
            Minus => {
                l.text(b"-");
            }
            _ => {
                l.text(&self.operator(t)).sp();
            }
        }
    }

    /// `(` arguments `)`: no space after `(` or before `)` and `,`, one after `,`.
    fn args_into(&self, n: Node<'_>, l: &mut Lay, drops: &HashSet<u32>) {
        for c in n.children() {
            match c {
                Element::Token(t) if t.kind == LParen => {
                    l.sp2().text(b"(").sp2();
                }
                Element::Token(t) if t.kind == RParen => {
                    if l.last() == Some(crate::apply::SP) {
                        l.set_last(crate::apply::SP2);
                    } else if l.last() != Some(crate::apply::SP2) {
                        l.sp2();
                    }
                    l.text(b")");
                }
                Element::Token(t) if t.kind == Comma => {
                    l.sp2().text(b",").sp();
                }
                // `INPUT$(n, #f)`: the `#` is an element of its own.
                Element::Token(t) if t.kind == Hash => {
                    l.text(b"#").sp();
                }
                Element::Token(_) => {}
                Element::Node(c) => self.expr_into(c, l, drops),
            }
        }
    }

    /// An operator token: a word operator of `isoperator` (`qb64pe.bas` 24518) through `SCase2$`, any other word as
    /// written (`ROOT` of a `CONST`), a symbol as it is (`<>` and `><` both `<>`, as `lineformat$` reads them).
    pub fn operator(&self, t: Tok) -> Vec<u8> {
        let text = self.tok(t);
        match t.kind {
            Ne => b"<>".to_vec(),
            Le => b"<=".to_vec(),
            Ge => b">=".to_vec(),
            Ident
                if [
                    "_ORELSE", "_ANDALSO", "IMP", "EQV", "XOR", "OR", "AND", "_NEGATE", "NOT", "MOD",
                ]
                .iter()
                .any(|w| text.eq_ignore_ascii_case(w.as_bytes())) =>
            {
                self.kw2(text)
            }
            _ => text.to_vec(),
        }
    }

    /// A literal as `lineformat$` keeps it.
    pub fn literal(&self, t: Tok) -> Vec<u8> {
        let text = self.tok(t);
        match t.kind {
            StringLit => {
                let mut s = text.to_vec();
                if s.len() < 2 || !s.ends_with(b"\"") {
                    s.push(b'"');
                }
                s
            }
            Number => number(text),
            _ => text.to_vec(),
        }
    }

    /// A node's text with the blanks at its ends left out (a form whose layout is the text as written).
    pub fn trimmed(&self, n: Node<'_>) -> Vec<u8> {
        let text = self.text(n.span());
        let start = text.iter().position(|c| !c.is_ascii_whitespace()).unwrap_or(text.len());
        let end = text
            .iter()
            .rposition(|c| !c.is_ascii_whitespace())
            .map_or(start, |e| e + 1);
        text[start..end].to_vec()
    }
}

/// A number as `lineformat$` keeps it in a layout: a plain decimal integer (no point, exponent or suffix) is its
/// digits without leading zeros; any other number its text in capitals.
pub fn number(text: &[u8]) -> Vec<u8> {
    if !text.is_empty() && text.iter().all(u8::is_ascii_digit) {
        let digits = text.iter().position(|&c| c != b'0').map_or(&b"0"[..], |i| &text[i..]);
        return digits.to_vec();
    }
    text.to_ascii_uppercase()
}

/// `isoperator` (`qb64pe.bas` 24518) of an element in capitals.
fn is_operator(e: &[u8]) -> bool {
    matches!(
        e,
        b"_ORELSE"
            | b"_ANDALSO"
            | b"IMP"
            | b"EQV"
            | b"XOR"
            | b"OR"
            | b"AND"
            | b"_NEGATE"
            | b"NOT"
            | b"="
            | b">"
            | b"<"
            | b"<>"
            | b"><"
            | b"<="
            | b"=<"
            | b">="
            | b"=>"
            | b"+"
            | b"-"
            | b"MOD"
            | b"\\"
            | b"*"
            | b"/"
            | b"^"
    )
}

#[cfg(test)]
mod tests {
    use super::number;

    #[test]
    fn numbers_as_lineformat_keeps_them() {
        // `lineformat$` (`qb64pe.bas` 24588): leading zeros culled off a plain integer, the rest as typed in capitals.
        assert_eq!(number(b"007"), b"7");
        assert_eq!(number(b"0"), b"0");
        assert_eq!(number(b"1.50"), b"1.50");
        assert_eq!(number(b"1e3"), b"1E3");
        assert_eq!(number(b"&hff"), b"&HFF");
        assert_eq!(number(b"5&&"), b"5&&");
    }
}
