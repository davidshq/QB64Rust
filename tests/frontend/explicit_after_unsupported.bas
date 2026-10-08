' TEST: check-fail
$CONSOLE:ONLY
' Under OPTION _EXPLICIT, an undeclared variable after a declaration marked "not supported yet" gets no error: the
' declaration may declare it, so a real error could be false (follow-on rule, m2-parser-breadth design D10, which
' replaced m2-control-flow-slice 4.3). `DIM AS LONG v` (upstream `arrays\t659_variable_boolean_expr`) starts it;
' the real error before it stays.
OPTION _EXPLICIT
a = 1
DIM AS LONG v
v = 2
DIM u AS _UNSIGNED LONG
u = 3
b = 4
SYSTEM
