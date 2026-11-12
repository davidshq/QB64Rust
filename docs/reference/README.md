# QB64Rust language reference

<!-- Written by tools/reference/reference.py render; do not edit. -->

One page per statement, function, keyword and metacommand. Each page says what QB64Rust does, which
errors it gives, and where it differs from QB64pe 4.7.0 and QuickBASIC 4.5. Every rule on a page names
its source, and every example is a test program whose output was recorded (`tools\reference\README.md`).

The examples start with `$CONSOLE:ONLY` so that their output can be recorded; the pages leave that line
out. Run in a window, an example prints the same text there.

28 pages written; the inventory names 503.

## By area

- **Lines, names, comments, labels, $INCLUDE, the preprocessor**: [$ERROR](entries/meta-error.md), [$ERRORLOCATION](entries/meta-errorlocation.md), [$IF](entries/meta-if.md), [$INCLUDE](entries/meta-include.md), [$INCLUDEONCE](entries/meta-includeonce.md), [$LET](entries/meta-let.md), [$NOPREFIX](entries/meta-noprefix.md), [REM](entries/rem.md)
- **Types, literals, operators, conversions**: [_ANDALSO](entries/_andalso.md), [_CAST](entries/_cast.md), [_NEGATE](entries/_negate.md), [_ORELSE](entries/_orelse.md), [AND](entries/and.md), [CDBL](entries/cdbl.md), [CINT](entries/cint.md), [CLNG](entries/clng.md), [CSNG](entries/csng.md), [EQV](entries/eqv.md), [IMP](entries/imp.md), [MOD](entries/mod.md), [NOT](entries/not.md), [OR](entries/or.md), [XOR](entries/xor.md)
- **Control flow**: [SELECT CASE](entries/select-case.md)
- **Strings**: [LEFT$](entries/left-str.md)
- **Screen modes and drawing**: [LINE](entries/line.md)
- **Memory**: [_MEMCOPY](entries/_memcopy.md)
- **Window, clipboard, system, date and time**: [FRE](entries/fre.md)

## Appendices

- [The auto-included constants](appendix-constants.md)
