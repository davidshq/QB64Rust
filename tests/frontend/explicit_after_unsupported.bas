' TEST: check-fail
$CONSOLE:ONLY
' Under OPTION _EXPLICIT, an undeclared variable after a construct marked "not supported yet" is only marked too
' (m2-control-flow-slice task 4.3): the construct may declare it, so a real error could be false. Marked by the
' parser (`DIM AS LONG v`, upstream `arrays\t659_variable_boolean_expr`) or by `sema` (`_UNSIGNED LONG`). The real
' error before them stays.
OPTION _EXPLICIT
a = 1
DIM AS LONG v
v = 2
DIM u AS _UNSIGNED LONG
u = 3
b = 4
SYSTEM
