$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): LBOUND((a)) of an array parameter.
DIM a(3 TO 7) AS LONG
s a()
SYSTEM
SUB s (a() AS LONG)
    PRINT LBOUND((a)); UBOUND((a))
END SUB
