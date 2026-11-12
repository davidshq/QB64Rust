# QB64Rust language reference

<!-- Written by tools/reference/reference.py render; do not edit. -->

One page per statement, function, keyword and metacommand. Each page says what QB64Rust does, which
errors it gives, and where it differs from QB64pe 4.7.0 and QuickBASIC 4.5. Every rule on a page names
its source, and every example is a test program whose output was recorded (`tools\reference\README.md`).

The examples start with `$CONSOLE:ONLY` so that their output can be recorded; the pages leave that line
out. Run in a window, an example prints the same text there.

63 pages written; the inventory names 499.

## By area

- **Lines, names, comments, labels, $INCLUDE, the preprocessor**: [$ERROR](entries/meta-error.md), [$ERRORLOCATION](entries/meta-errorlocation.md), [$IF](entries/meta-if.md), [$INCLUDE](entries/meta-include.md), [$INCLUDEONCE](entries/meta-includeonce.md), [$LET](entries/meta-let.md), [$NOPREFIX](entries/meta-noprefix.md), [REM](entries/rem.md)
- **Types, literals, operators, conversions**: [_ANDALSO](entries/_andalso.md), [_BIT](entries/_bit.md), [_BYTE](entries/_byte.md), [_CAST](entries/_cast.md), [_DEFINE](entries/_define.md), [_FLOAT](entries/_float.md), [_INTEGER64](entries/_integer64.md), [_NEGATE](entries/_negate.md), [_OFFSET](entries/_offset.md), [_ORELSE](entries/_orelse.md), [_UNSIGNED](entries/_unsigned.md), [AND](entries/and.md), [CDBL](entries/cdbl.md), [CINT](entries/cint.md), [CLNG](entries/clng.md), [CSNG](entries/csng.md), [DEFINT](entries/defint.md), [DOUBLE](entries/double.md), [EQV](entries/eqv.md), [IMP](entries/imp.md), [INTEGER](entries/integer.md), [LONG](entries/long.md), [MOD](entries/mod.md), [NOT](entries/not.md), [OR](entries/or.md), [SINGLE](entries/single.md), [STRING](entries/string.md), [XOR](entries/xor.md)
- **Variables, scope, constants**: [COMMON](entries/common.md), [CONST](entries/const.md), [DIM](entries/dim.md), [LET](entries/let.md), [$COLOR](entries/meta-color.md), [OPTION](entries/option.md), [SHARED](entries/shared.md), [STATIC](entries/static.md), [SWAP](entries/swap.md)
- **Control flow**: [_CONTINUE](entries/_continue.md), [DO](entries/do.md), [END](entries/end.md), [EXIT](entries/exit.md), [FOR](entries/for.md), [GOSUB](entries/gosub.md), [GOTO](entries/goto.md), [IF](entries/if.md), [ON](entries/on.md), [RUN](entries/run.md), [SELECT CASE](entries/select-case.md), [STOP](entries/stop.md), [SYSTEM](entries/system.md), [WHILE](entries/while.md)
- **Strings**: [LEFT$](entries/left-str.md)
- **Screen modes and drawing**: [LINE](entries/line.md)
- **Memory**: [_MEMCOPY](entries/_memcopy.md)
- **Window, clipboard, system, date and time**: [FRE](entries/fre.md)

## Appendices

- [The auto-included constants](appendix-constants.md)
