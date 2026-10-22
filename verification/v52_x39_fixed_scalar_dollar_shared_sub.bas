$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x39_fixed_scalar_dollar_shared_sub)
' name$ in a SUB of a DIM SHARED STRING * 1 scalar, as a place and as a value (QB64pe file.bas, pathsep$).
DIM SHARED pathsep AS STRING * 1
s
PRINT "["; pathsep; "]"
SYSTEM
SUB s
    pathsep$ = "\"
    d$ = "a\b"
    IF RIGHT$(d$, 1) <> pathsep$ THEN d$ = d$ + pathsep$
    PRINT d$; INSTR(d$, pathsep$); LEN(pathsep$)
END SUB
