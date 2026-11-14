# Built-in statements

The built-in statements are written with their arguments after the name, without parentheses. Each raises its error at
the end of the statement, where a handler can take it.

The pages of this area: [SWAP](../entries/swap.md), [MID$](../entries/mid-str.md),
[RANDOMIZE](../entries/randomize.md), [SHELL](../entries/shell.md), [ENVIRON](../entries/environ.md),
[CHAIN](../entries/chain.md), [LPRINT](../entries/lprint.md), [CLEAR](../entries/clear.md),
[DATE$](../entries/date-str.md), [TIME$](../entries/time-str.md), [LET](../entries/let.md).

## Forms

A statement whose forms are read by a template also compiles after `CALL`, with one pair of parentheses around all
that follows its name.
<!-- src: language/builtin-statements "A statement with a template after CALL" -->

An error a statement raises is serviced at its end: the handler runs, and `RESUME NEXT` goes on with the next
statement.
<!-- src: language/builtin-statements "Runtime errors of built-in statements" -->

`DATE$ = text$` and `TIME$ = text$` compile and change nothing: the system's date and time stay as they were.
<!-- src: language/builtin-statements "DATE$ and TIME$ as statements"; tests\reference\examples\date-str_1 -->
