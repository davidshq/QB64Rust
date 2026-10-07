' TEST: check-fail
$CONSOLE:ONLY
' OPTION _EXPLICIT inside an IF block applies to the whole program (m2-control-flow-slice D7,
' verification\v17_f_explicit_cond_jump): `x` after the block is not declared. Until `IF` is supported (task
' 5.2) it is only marked, after the marked block; the mark shows the OPTION inside the block was found.
IF 1 THEN
    OPTION _EXPLICIT
END IF
x = 1
PRINT x
SYSTEM
