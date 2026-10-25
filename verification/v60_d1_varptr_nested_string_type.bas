$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): VARPTR and VARSEG of the members of a TYPE variable whose nested member
' holds a STRING (the strings are not used).
TYPE inner
    s AS STRING
    v AS LONG
END TYPE
TYPE outer
    a AS INTEGER
    n AS inner
    z AS DOUBLE
END TYPE
DIM o AS outer
PRINT VARPTR(o.a); VARPTR(o.n.s); VARPTR(o.n.v); VARPTR(o.z)
PRINT VARSEG(o); VARSEG(o.n.v)
o.z = 2.5: o.n.v = 7
PRINT o.z; o.n.v
SYSTEM
