' TEST: ir
' The statements of the last built-ins of the upstream report (m4-builtins-param-types design D1, D4): _CLIPBOARD$ =
' as the call of its row; CLEAR as sub_clear with its three ignored arguments and their passed bits, bare and
' after CALL.
$CONSOLE:ONLY
DIM k AS LONG, s AS STRING, f AS STRING * 4
_CLIPBOARD$ = s
_CLIPBOARD$ = f
CLEAR
CLEAR k, , 100
CLEAR , 2
CALL CLEAR
SYSTEM
