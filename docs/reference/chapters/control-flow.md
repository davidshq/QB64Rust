# Control flow

Blocks choose and repeat: `IF`, `SELECT CASE`, `FOR`, `DO` and `WHILE`. Jumps go to a label of the same body with
`GOTO`, `GOSUB` and `ON`. `END`, `SYSTEM`, `STOP` and `RUN` end or restart the program.

The pages of this area: [IF](../entries/if.md), [SELECT CASE](../entries/select-case.md), [FOR](../entries/for.md),
[DO](../entries/do.md), [WHILE](../entries/while.md), [EXIT](../entries/exit.md),
[_CONTINUE](../entries/_continue.md), [GOTO](../entries/goto.md), [GOSUB](../entries/gosub.md),
[ON](../entries/on.md), [END](../entries/end.md), [SYSTEM](../entries/system.md), [STOP](../entries/stop.md),
[RUN](../entries/run.md).

## Conditions

A condition is true when its value is not 0. A string condition is a compile error.
<!-- src: language/control-flow "IF"; language/control-flow "DO and WHILE loops" -->

`SELECT CASE` runs the first `CASE` with a matching item; `SELECT EVERYCASE` runs every one. An item is a value, a
range `a TO b` or a comparison `IS op v`, compared with the selector as QB64pe compares them.
<!-- src: language/control-flow "SELECT CASE"; language/control-flow "SELECT EVERYCASE"; tests\reference\examples\select-case_1 -->

## Loops

A `FOR` loop computes its start, limit and step once, and runs while the variable has not passed the limit: a loop
from 5 to 1 does not run. Its test uses a type wider than the variable, as QB64pe's does.
<!-- src: language/control-flow "FOR loops"; tests\reference\examples\for_1 -->

`EXIT FOR`, `EXIT DO` and `EXIT WHILE` leave the innermost loop of that kind; `_CONTINUE` goes on with the next pass
of the innermost loop, its step and test included.
<!-- src: language/control-flow "EXIT from loops"; language/control-flow "_CONTINUE" -->

## Labels and jumps

A label, or a line number, belongs to the body it stands in: the main module or one procedure. A jump to a label of
another body is a compile error.
<!-- src: language/control-flow "Labels per body"; language/control-flow "Line numbers" -->

The program has one stack of pending `GOSUB`s. `RETURN` with none pending raises error 3.
<!-- src: language/control-flow "GOSUB and RETURN" -->

`ON n GOTO` and `ON n GOSUB` go to the n-th label, and on with the next statement for 0 or an `n` past the list; a
negative `n` raises error 5.
<!-- src: language/control-flow "ON GOTO and ON GOSUB" -->

## Ending and starting again

`END n` and `SYSTEM n` end the program with exit code `n`. `STOP` ends it as `SYSTEM` does.
<!-- src: language/control-flow "END and SYSTEM with an exit code"; language/control-flow "STOP" -->

`RUN` starts the program again from its first line, or from a label of the main module, after resetting the screen and
clearing every variable as `CLEAR` does.
<!-- src: language/control-flow "RUN" -->
