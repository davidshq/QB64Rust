' TEST: check-ok
$CONSOLE:ONLY
' VARPTR, VARSEG and SADD of a procedure's variables and parameters, CALL ABSOLUTE and TYPE members named like
' keywords and built-ins (qbasic-testcases design D1-D4; verification\v61_*): a local, a STATIC one, a local
' array, a string; a scalar parameter called with values, an element and a member; a TYPE parameter and an
' array parameter passed on; CALL ABSOLUTE with a variable, a value and a string's characters as the offset.
TYPE st
    time AS INTEGER
    case AS INTEGER
    len AS LONG
END TYPE
DIM s AS st, a(3) AS INTEGER
DIM SHARED click AS INTEGER
s.time = 4: PRINT s.time; TIME$ <> ""
t 3
t (s.time)
t a(2)
t s.len
outer s, a()
loc
SYSTEM

SUB loc
    DIM k AS INTEGER, w(1 TO 3) AS INTEGER
    STATIC st AS INTEGER
    STATIC mx AS STRING
    PRINT VARPTR(k); VARSEG(k); VARPTR(st); VARPTR(w(2)); VARSEG(w(1))
    IF mx = "" THEN mx = CHR$(&HCB)
    CALL ABSOLUTE(2, 2.5#, click, SADD(mx))
END SUB

SUB t (p AS INTEGER)
    PRINT VARPTR(p); VARSEG(p)
END SUB

SUB outer (x AS st, z() AS INTEGER)
    inner x
    fill z()
END SUB

SUB inner (y AS st)
    PRINT VARPTR(y.len); VARSEG(y.case)
END SUB

SUB fill (z() AS INTEGER)
    PRINT VARPTR(z(1)); VARSEG(z(1))
END SUB
