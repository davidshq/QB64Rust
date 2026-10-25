$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): VARPTR of a member of a TYPE variable whose nested member holds a STRING,
' then that string used (the old compiler leaves the strings of the value out: D-037).
TYPE inner
    s AS STRING
    v AS LONG
END TYPE
TYPE outer
    a AS INTEGER
    n AS inner
END TYPE
DIM o AS outer
PRINT VARPTR(o.n.v)
o.n.s = "hello"
PRINT o.n.s; LEN(o.n.s)
SYSTEM
