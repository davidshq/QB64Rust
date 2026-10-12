' TEST: typed
$CONSOLE:ONLY
' Uses of constants become literal nodes (m2-control-flow-slice D6): an integer constant is _INTEGER64 (so
' `i3 * 1000000000` is a 64-bit operation), a float one DOUBLE, a suffix on the name or the use gives that type
' (a SINGLE literal held as a DOUBLE), a string constant is a string, a float beyond _INTEGER64 range stays
' DOUBLE; a parameter hides a main constant; a constant passed to a SUB is a copy.
CONST i3 = 3, third = 1 / 3, h% = 2.5, s = "ab"
PRINT i3 * 1000000000; third; h%; s
PRINT i3!; i3&; third%; h
CONST c3s! = 1 / 3, f## = 0.5, big = 1E+19 / 1
PRINT c3s!; f##; big
sb i3
END

SUB sb (i3)
    PRINT i3
END SUB
