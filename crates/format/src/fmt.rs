//! What every layout rule reads: the program, the file's bytes, the keyword style; and the spelling of keywords,
//! built-in names and user names (design D3).

use crate::lay::{scase, scase2, word};
use qb64rust_base::layout::{KeywordStyle, Settings};
use qb64rust_base::{FileId, SourceMap, Span};
use qb64rust_sema::{Program, SymbolKind};
use qb64rust_syntax::tree::Tok;

pub(crate) struct Fmt<'a> {
    pub map: &'a SourceMap,
    pub program: &'a Program,
    pub file: FileId,
    pub settings: &'a Settings,
}

/// The bytes of a type suffix: what follows a name (`$`, `%`, `&&`, `~%`, `` `8 ``, `$16` of a fixed string).
fn is_suffix_byte(c: u8) -> bool {
    matches!(c, b'$' | b'%' | b'&' | b'!' | b'#' | b'~' | b'`')
}

/// The length of the name part of a name as written: up to its type suffix.
pub(crate) fn name_len(text: &[u8]) -> usize {
    text.iter().position(|&c| is_suffix_byte(c)).unwrap_or(text.len())
}

impl<'a> Fmt<'a> {
    pub fn style(&self) -> KeywordStyle {
        self.settings.keywords
    }

    pub fn text(&self, span: Span) -> &'a [u8] {
        self.map.text(span)
    }

    pub fn tok(&self, t: Tok) -> &'a [u8] {
        self.map.text(t.span)
    }

    /// `SCase$` of a keyword phrase written in capitals with blanks between its words (`END IF`, `ON ERROR GOTO`):
    /// each word's spelling from QB64pe's literals, the blanks as they are. A word the table lacks is spelled by
    /// `SCase2$`'s rule.
    pub fn kw(&self, upper: &str) -> Vec<u8> {
        self.kw_bytes(upper.as_bytes())
    }

    /// [`Fmt::kw`] of bytes.
    pub fn kw_bytes(&self, upper: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        for (i, w) in upper.split(|&c| c == b' ').enumerate() {
            if i > 0 {
                out.push(b' ');
            }
            match word(w) {
                Some(s) => out.extend(scase(s.as_bytes(), self.style())),
                None => out.extend(scase2(w, self.style())),
            }
        }
        out
    }

    /// The keyword a token spells, by its text in capitals.
    pub fn kw_tok(&self, t: Tok) -> Vec<u8> {
        self.kw_bytes(&self.tok(t).to_ascii_uppercase())
    }

    /// `SCase2$` of a text.
    pub fn kw2(&self, text: &[u8]) -> Vec<u8> {
        scase2(text, self.style())
    }

    /// The name a built-in is registered with (`InStr`, `_RGB32`), in the keyword style, when `bare` (no suffix,
    /// any case) names one.
    pub fn builtin(&self, bare: &[u8]) -> Option<Vec<u8>> {
        if let Some(b) = qb64rust_builtins::find_any(bare).next() {
            return Some(scase(b.name.as_bytes(), self.style()));
        }
        // OpenGL's routines are internal SUBs and FUNCTIONs of the old compiler (`_glBegin`), its constants are
        // constants as it registers them (`_GL_TRIANGLES`).
        let gl = &qb64rust_sema::gl_table::ROUTINES;
        if let Some(r) = gl.iter().find(|r| r.name.as_bytes().eq_ignore_ascii_case(bare)) {
            return Some(scase(r.name.as_bytes(), self.style()));
        }
        let consts = &qb64rust_sema::gl_table::CONSTANTS;
        consts
            .iter()
            .find(|c| c.name.as_bytes().eq_ignore_ascii_case(bare))
            .map(|c| c.name.as_bytes().to_vec())
    }

    /// A name token as QB64pe writes it (design D3): each name the symbol table records inside it with the spelling
    /// of its symbol's definition, the suffix as written; a built-in with its registered spelling; any other name
    /// as written.
    pub fn name(&self, t: Tok) -> Vec<u8> {
        let text = self.tok(t);
        let mut syms: Vec<_> = self.program.symbols.within(t.span).collect();
        // A token inside a name of several tokens (`q` of `p . q`): the symbol that starts before it.
        if syms.is_empty()
            && let Some(id) = self.program.symbols.at(t.span.file, t.span.start)
        {
            let sym = self.program.symbols.get(id);
            if let Some(&span) = std::iter::once(&sym.def)
                .chain(&sym.refs)
                .find(|s| s.file == t.span.file && s.start <= t.span.start && t.span.start < s.end)
            {
                syms.push((span, id));
            }
        }
        // An OpenGL routine is a procedure to `sema` with its first call as the definition; the old compiler spells it
        // as it registers it.
        let gl_proc = syms.len() == 1
            && matches!(self.program.symbols.get(syms[0].1).kind, SymbolKind::Proc(_))
            && qb64rust_sema::opengl::gl_routine(&ascii_upper(&text[..name_len(text)])).is_some();
        if syms.is_empty() || gl_proc {
            let n = name_len(text);
            if !text[..n].contains(&b'.')
                && let Some(b) = self.builtin(&text[..n])
            {
                let mut out = b;
                out.extend_from_slice(&text[n..]);
                return out;
            }
            return text.to_vec();
        }
        let mut out = text.to_vec();
        // Each part of each name as its symbol's definition spells that part; from the last part to the first, so
        // that the offsets of the earlier ones stay (a spelling has the length of the name it replaces).
        let mut edits: Vec<(usize, usize, Vec<u8>)> = Vec::new();
        for &(span, id) in &syms {
            let sym = self.program.symbols.get(id);
            let mut def = self.text(sym.def);
            // A FUNCTION's result inside its body is the function's name (`id.cn`): its header's spelling.
            if let SymbolKind::Var(v) = sym.kind
                && let qb64rust_sema::Storage::Result(q) = self.program.var(v).storage
                && let Some((_, proc)) = self.program.symbols.iter().find(|(_, s)| s.kind == SymbolKind::Proc(q))
            {
                def = self.text(proc.def);
            }
            // QB64pe's own functions behind a keyword (`_IKW_DecodeURL$` for `_DECODEURL$`) show the keyword.
            if def.len() > 5 && def[..5].eq_ignore_ascii_case(b"_IKW_") {
                def = &def[4..];
            }
            let ref_parts = parts(self.text(span), span.start);
            let def_parts = parts(def, 0);
            // The members of `_MEM`, which no `TYPE` block defines, as the old compiler registers them: in capitals.
            let mem = match sym.kind {
                SymbolKind::Var(v) => matches!(self.program.var(v).ty,
                    qb64rust_sema::Ty::User(u) if self.program.user_type(u).name == qb64rust_sema::MEM_TYPE),
                _ => false,
            };
            for (k, &(at, len)) in ref_parts.iter().enumerate() {
                if at < t.span.start || at + len > t.span.end {
                    continue;
                }
                let from = (at - t.span.start) as usize;
                match def_parts.get(k) {
                    Some(&(d_at, d_len)) => {
                        edits.push((from, len as usize, def[d_at as usize..(d_at + d_len) as usize].to_vec()));
                    }
                    None if mem => {
                        edits.push((from, len as usize, text[from..from + len as usize].to_ascii_uppercase()));
                    }
                    None => {}
                }
            }
        }
        edits.sort_by_key(|e| e.0);
        for (from, len, with) in edits.into_iter().rev() {
            out.splice(from..from + len, with);
        }
        out
    }
}

/// A name's bytes in capitals as a `String`: a name is ASCII (letters, digits, `_` and `.`), so each byte is its
/// character.
pub(crate) fn ascii_upper(name: &[u8]) -> String {
    name.iter().map(|&b| char::from(b.to_ascii_uppercase())).collect()
}

/// The parts of a name split at its dots, each as (offset from `base`, length): the blanks around a part and the
/// type suffix of the last left out (`p . q%` gives `p` and `q`).
fn parts(text: &[u8], base: u32) -> Vec<(u32, u32)> {
    let n = name_len(text);
    let mut out = Vec::new();
    let mut start = 0;
    for end in text[..n]
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == b'.')
        .map(|(i, _)| i)
        .chain([n])
    {
        let part = &text[start..end];
        let lead = part.iter().take_while(|c| c.is_ascii_whitespace()).count();
        let trail = part.iter().rev().take_while(|c| c.is_ascii_whitespace()).count();
        let len = part.len().saturating_sub(lead + trail);
        let to = |x: usize| u32::try_from(x).expect("a name is short");
        out.push((base + to(start + lead), to(len)));
        start = end + 1;
    }
    out
}
