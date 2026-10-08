' TEST: check-fail
$CONSOLE:ONLY
' Procedure errors (m2-procedures-and-errors D8; old compiler's messages in verification\v14_err_*): one per
' statement, each statement checked on its own. The declarations marked "not supported yet" stand last: a marked
' declaration drops the real errors after it (the follow-on rule of m2-parser-breadth, design D10).
DIM n AS LONG: DIM SHARED w AS LONG
s 1, 2
CALL s
s (1, 2)
s "x"
t 1
x = f&("a")
EXIT SUB
EXIT FUNCTION
PRINT s
PRINT f%(1)
f 1
nothere 1
CLS
y = 1
SUB t (a AS STRING)
END SUB
FUNCTION f& (a AS LONG)
    f& = "x"
    f! = 1
END FUNCTION
SUB y
END SUB
SUB v$
END SUB
SUB s (a AS LONG)
    DIM SHARED z
    STATIC w
END SUB
SUB s
END SUB
SHARED n
STATIC q
