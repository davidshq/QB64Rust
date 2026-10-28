' TEST: check-fail
$CONSOLE:ONLY
' Errors of qbasic-testcases (design D2-D4; verification\v61_px*, c5-c7, m_*): a plain variable passed to a
' parameter whose address is taken (the old compiler's internal error), a main-module one, a shared one, a string,
' and a SUB's parameter passed on; ABSOLUTE without CALL and CALL ABSOLUTE without parentheses ("Syntax error",
' "Expected CALL sub-name (...)"); a TYPE member named PRINT ("Name already in use") and one with a single leading
' underscore ("Invalid name"). None is "not supported yet".
TYPE r1
    print AS LONG
END TYPE
TYPE r2
    _mine AS LONG
END TYPE
DIM x AS INTEGER, code(1) AS INTEGER
DIM SHARED g AS INTEGER
t x
t g
u g$
a 5
ABSOLUTE (VARPTR(code(0)))
CALL ABSOLUTE VARPTR(code(0))
SYSTEM

SUB a (y AS INTEGER)
    t y
END SUB

SUB t (p AS INTEGER)
    PRINT VARPTR(p)
END SUB

SUB u (s$)
    PRINT SADD(s$)
END SUB
