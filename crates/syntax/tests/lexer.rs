//! Token snapshots (task 3.1) and byte coverage of the lexer.

use qb64rust_syntax::dump_tokens;
use qb64rust_syntax::lexer::tokenize;

#[test]
fn literals_and_suffixes() {
    insta::assert_snapshot!(dump_tokens(
        b"x = 12 + 1.5E-3 + 5&& + &HFF& + .5 + 1. + 1D5 + 1.5## + 7! + 2# + &O17 + &B101 + 3~%\n"
    ));
}

/// The suffixes of `m2-numeric-types` on names and numbers; `$n` only on a name.
#[test]
fn new_type_suffixes() {
    insta::assert_snapshot!(dump_tokens(
        b"u~%% = 300~%% + b`3 + c~` + o%& + q~%& + 9`64 + -1~&& + s$12 + LEFT$(t$, 2) + 5$\n"
    ));
}

#[test]
fn names_keywords_and_operators() {
    insta::assert_snapshot!(dump_tokens(
        b"DIM a AS _INTEGER64, b$, c.d%: LET e## = -a MOD 2 <> b =< c >< d\r\n?\"x\";\"y\" , x"
    ));
}

#[test]
fn comments_metacommands_continuations() {
    insta::assert_snapshot!(dump_tokens(
        b"$CONSOLE:ONLY\nREM all of this: PRINT\nPRINT 1 ' rest \"x\nPRINT 1 + _\n  2: REMARK = 3\r\"open\n_x\rend"
    ));
}

/// `DATA` items are one `DataText` token up to the line end or a `:` outside quotes; `'`, `REM` and inner `"` are
/// data (measured M2, `verification\v16_m2_*`); an inner `"` does not protect a `:`, a leading one does. Also after
/// `ELSE` and in lower case; `DATA:` and `DATA` alone have no `DataText`; `DATA$` and `a(1).data` are names.
#[test]
fn data_text() {
    insta::assert_snapshot!(dump_tokens(
        b"DATA   x  ,  y y  ,,\"  q  \",\nDATA a'b, c REM d\ndata \"a:b\", c: PRINT 1\nDATA a\"b: x\nIF 1 THEN 2 ELSE DATA 1,&h5\nDATA:DATA\nDATA$ = a(1).data\n"
    ));
}

/// Every byte value 0x01-0xFF inside a string literal and inside a comment stays inside that one token.
#[test]
fn every_byte_in_string_and_comment() {
    let mut body: Vec<u8> = (1u8..=255).filter(|&b| b != b'"' && b != b'\r' && b != b'\n').collect();
    let mut src = b"PRINT \"".to_vec();
    src.append(&mut body.clone());
    src.extend_from_slice(b"\"\n' ");
    let comment_start = src.len() - 2;
    src.append(&mut body);
    src.extend_from_slice(b"\n");
    let toks = tokenize(&src);
    let kinds: Vec<_> = toks.iter().map(|t| format!("{:?}", t.kind)).collect();
    assert_eq!(
        kinds,
        ["Ident", "Whitespace", "StringLit", "Newline", "Comment", "Newline"]
    );
    assert_eq!(toks[2].len as usize, 252 + 2); // 0x01-0xFF without `"`, CR and LF, plus the quotes
    assert_eq!(toks[4].len as usize, src.len() - 1 - comment_start);
    assert_eq!(toks.iter().map(|t| t.len as usize).sum::<usize>(), src.len());
}

/// Bytes outside strings and comments that start no token become single-byte `Unknown` tokens; nothing is lost.
#[test]
fn every_byte_alone_is_covered() {
    for b in 0u8..=255 {
        let src = [b'x', b, b'y', b];
        let total: u32 = tokenize(&src).iter().map(|t| t.len).sum();
        assert_eq!(total, 4, "byte {b:#04x}");
    }
}
