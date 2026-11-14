# DATA, READ and RESTORE

The `DATA` statements of a program form one list of items, which `READ` takes one by one and `RESTORE` starts again.

The pages of this area: [DATA](../entries/data.md), [READ](../entries/read.md), [RESTORE](../entries/restore.md).

## The list

The items of every `DATA` statement, in the main module and the procedures, form one list in file order. Blanks around
an unquoted item are dropped; a quoted item keeps them.
<!-- src: language/data-read "The program's data"; tests\reference\examples\data_1 -->

`READ` stores the next item into each target, converted to its type. With no item left it raises error 4; a quoted
item read into a number raises error 2.
<!-- src: language/data-read "READ" -->

`RESTORE label` starts the next `READ` at the first `DATA` after the label, wherever the label stands.
<!-- src: language/data-read "RESTORE"; tests\reference\examples\restore_1 -->
