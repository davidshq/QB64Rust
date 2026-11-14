# Built-in functions

The built-in functions are QuickBASIC's and QB64pe's: strings, math, conversions, files, the system. Each converts its
arguments and types its result as QB64pe does.

The pages of this area: [LEN](../entries/len.md), [LEFT$](../entries/left-str.md), [MID$](../entries/mid-str.md),
[INSTR](../entries/instr.md), [STR$](../entries/str-str.md), [VAL](../entries/val.md), [CHR$](../entries/chr-str.md),
[ASC](../entries/asc.md), [ABS](../entries/abs.md), [INT](../entries/int.md), [SQR](../entries/sqr.md),
[RND](../entries/rnd.md), [_IIF](../entries/_iif.md), [_MIN](../entries/_min.md), [_CAST](../entries/_cast.md),
[_SHL](../entries/_shl.md), [_ARCSEC](../entries/_arcsec.md), [_ASIN](../entries/_asin.md),
[_ASINH](../entries/_asinh.md), [_ATAN2](../entries/_atan2.md), [_BIN$](../entries/_bin-str.md),
[_CEIL](../entries/_ceil.md), [_CLAMP](../entries/_clamp.md), [_CV](../entries/_cv.md), [_D2R](../entries/_d2r.md),
[_HYPOT](../entries/_hypot.md), [_INSTRREV](../entries/_instrrev.md), [_MK$](../entries/_mk-str.md),
[_PI](../entries/_pi.md), [_ROL](../entries/_rol.md), [_ROUND](../entries/_round.md), [_SEC](../entries/_sec.md),
[_SECH](../entries/_sech.md), [_SETBIT](../entries/_setbit.md), [_SINH](../entries/_sinh.md),
[_STRCMP](../entries/_strcmp.md), [_TOSTR$](../entries/_tostr-str.md), [_TRIM$](../entries/_trim-str.md),
[ATN](../entries/atn.md), [CDBL](../entries/cdbl.md), [COS](../entries/cos.md), [CSNG](../entries/csng.md),
[CVI](../entries/cvi.md), [CVSMBF](../entries/cvsmbf.md), [EXP](../entries/exp.md), [FIX](../entries/fix.md),
[HEX$](../entries/hex-str.md), [INPUT$](../entries/input-str.md), [LCASE$](../entries/lcase-str.md),
[LOG](../entries/log.md), [LTRIM$](../entries/ltrim-str.md), [MKI$](../entries/mki-str.md),
[MKSMBF$](../entries/mksmbf-str.md), [OCT$](../entries/oct-str.md), [RIGHT$](../entries/right-str.md),
[RTRIM$](../entries/rtrim-str.md), [SGN](../entries/sgn.md), [SIN](../entries/sin.md),
[SPACE$](../entries/space-str.md), [STRING$](../entries/string-str.md), [TAN](../entries/tan.md),
[UCASE$](../entries/ucase-str.md), [FRE](../entries/fre.md), [SETMEM](../entries/setmem.md).

## Names and arguments

A built-in is named in any letter case, with its required suffix (`LEFT$`) or without one where it has none (`LEN`).
Too few or too many arguments, or a string for a number, is a compile error.
<!-- src: language/builtin-functions "Names and arity" -->

Each argument is converted to its slot's type as QB64pe converts it: a float to an integer slot rounded half to even.
<!-- src: language/builtin-functions "Argument conversion" -->

A built-in with no required argument may be written without parentheses: `RND`, `TIMER`, `COMMAND$`.
<!-- src: language/builtin-functions "Built-ins without parentheses" -->

## Errors

An argument outside a built-in's domain raises QB64pe's error: 5 for `ASC("")`, `SQR(-1)` or `LOG(0)`, 6 for
`CINT(40000)`.
<!-- src: language/builtin-functions "Runtime errors of built-ins"; tests\reference\examples\cint_1 -->

A name with a leading underscore that calls no built-in, and a built-in function used as a statement, are compile
errors.
<!-- src: language/builtin-functions "Names that call no built-in" -->
