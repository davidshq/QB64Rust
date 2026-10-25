$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): VARPTR of the members of a TYPE with two STRING members, and PEEK and
' POKE of a number member through them (the strings are not used).
TYPE ts
    s1 AS STRING
    v AS INTEGER
    s2 AS STRING
    w AS LONG
END TYPE
DIM x AS ts
PRINT VARPTR(x.s1); VARPTR(x.v); VARPTR(x.s2); VARPTR(x.w)
x.v = 258
DEF SEG = VARSEG(x)
PRINT PEEK(VARPTR(x.v)); PEEK(VARPTR(x.v) + 1)
POKE VARPTR(x.w), 9
PRINT x.w
DEF SEG
SYSTEM
