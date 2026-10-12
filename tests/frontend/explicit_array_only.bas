' TEST: check-ok
$CONSOLE:ONLY
' OPTION _EXPLICITARRAY alone leaves implicit scalars allowed (m2-control-flow-slice D7,
' verification\v17_f_explicitarray_scalar).
OPTION _EXPLICITARRAY
x = 1
PRINT x
SYSTEM
