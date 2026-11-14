# Default types

A name with no suffix and no `AS` clause takes its type from its first letter. `DEFINT` and its kin, and `_DEFINE`,
set the type a letter gives; without them it is SINGLE.

The pages of this area: [DEFINT](../entries/defint.md), [_DEFINE](../entries/_define.md).

## The rule

`DEFINT`, `DEFLNG`, `DEFSNG`, `DEFDBL` and `DEFSTR` set the type of letters and letter ranges, without regard to case;
`_DEFINE letters AS type` does so for any type a suffix can give.
<!-- src: language/default-types "Default type by first letter"; language/default-types "_DEFINE"; tests\reference\examples\defint_1 -->

A statement applies to every line below it in the file, whether it runs or not and whatever block or procedure it
stands in.
<!-- src: language/default-types "Default types apply from the statement on" -->

The default types a variable, an array, a parameter and a FUNCTION's result, but not a `CONST`, which takes the type
of its value.
<!-- src: language/default-types "What a default type gives its type to" -->
