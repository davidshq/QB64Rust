$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): under $UNSTABLE:TYPEFIELDS, a _STATIC member array and an
' unmarked one beside a _DYNAMIC member: LEN of a type without _DYNAMIC, REDIM of the inline members, a copy,
' REDIM of a parent whose type mixes the three, OPTION BASE 1 for the first bounds of each.
OPTION BASE 1
TYPE t
    s(3) _STATIC AS LONG
    u(3) AS LONG
    d(3) _DYNAMIC AS LONG
    vs(2) _STATIC AS STRING
END TYPE
TYPE plain
    s(3) _STATIC AS LONG
    u(0 TO 2) AS INTEGER
END TYPE
ON ERROR GOTO h
DIM x AS t, y AS t, p AS plain
PRINT "1. bounds:"; LBOUND(x.s); UBOUND(x.s); LBOUND(x.u); UBOUND(x.u); LBOUND(x.d); UBOUND(x.d); LBOUND(x.vs); UBOUND(x.vs)
PRINT "2. LEN(p):"; LEN(p)
x.s(2) = 2: x.u(2) = 3: x.d(2) = 4: x.vs(1) = "one"
REDIM _PRESERVE x.s(1 TO 3) AS LONG
PRINT "3. REDIM _PRESERVE of _STATIC with the same bounds:"; x.s(2)
REDIM x.s(1 TO 3) AS LONG
PRINT "4. plain REDIM of _STATIC with the same bounds:"; x.s(2)
y = x
x.vs(1) = "two"
PRINT "5. copy:"; y.u(2); y.d(2); " ["; y.vs(1); "]"
REDIM q(1 TO 2) AS t
q(2).s(3) = 33: q(2).d(3) = 34: q(2).vs(2) = "q2"
REDIM _PRESERVE q(1 TO 3) AS t
PRINT "6. _PRESERVE parent:"; q(2).s(3); q(2).d(3); " ["; q(2).vs(2); "]"; LBOUND(q(3).d); UBOUND(q(3).d)
REDIM q(1 TO 2) AS t
PRINT "7. plain REDIM parent:"; q(2).s(3); q(2).d(3); " ["; q(2).vs(2); "]"
SYSTEM
h:
PRINT "[error"; ERR; "at"; _ERRORLINE; "]";
RESUME NEXT
