' TEST: typed
$CONSOLE:ONLY
' Member arrays in the typed tree (m2-defxxx-common-arrays task 9.1, design D8): both declaration forms with
' their bounds (OPTION BASE as it stands at the block: 1 for the first three types, 0 for the last), an element
' through a variable, through an array element and through a chain, LBOUND and UBOUND of a member array (also
' with empty parentheses), LEN, REDIM and ERASE of one as a Clear (also REDIM SHARED and ERASE with empty
' parentheses), REDIM _PRESERVE as nothing, an element by reference and as an operand of SWAP (a member of an
' element too), a whole member array as an array argument, a whole value of an element.
OPTION BASE 1
TYPE inner
    AS INTEGER q(2), r
END TYPE
TYPE outer
    AS inner n(0 TO 1), m
    s(0 TO 1) AS STRING
END TYPE
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    g(2, 0 TO 3) AS _BYTE
END TYPE
OPTION BASE 0
TYPE t0
    w(3) AS DOUBLE
END TYPE
DIM t AS t1, a(2) AS t1, x(0) AS outer, z AS t0, i AS LONG, k AS LONG
t.v(2) = 7
a(i).v(i + 1) = t.v(2)
x(0).n(1).q(2) = 77
x(0).m.r = 88
x(0).s(1) = "s"
t.g(2, 3) = 1.5
z.w(i) = t.g(1, 0)
PRINT LBOUND(t.v); UBOUND(a(i).v); LBOUND(x(0).n(1).q); UBOUND(t.g, 2); UBOUND(t.g, k); UBOUND(t.v())
PRINT LEN(t); LEN(t.v(1)); LEN(x(0).n(1)); LEN(z)
REDIM t.v(0 TO 3)
REDIM a(i).v(3) AS LONG
REDIM _PRESERVE a(i).v(0 TO 3)
REDIM x(0).n(1).q(1 TO 2)
REDIM SHARED t.v(0 TO 3), t.g(1 TO 2, 0 TO 3)
ERASE t.g, t.v()
ERASE x(0).s
bump t.v(2)
bump (t.v(2))
SWAP t.v(0), a(1).v(1)
SWAP a(0).id, a(1).id
fill a(i).v()
names x(0).s()
x(0).n(0) = x(0).n(1)
SYSTEM

SUB bump (n AS LONG)
    n = n + 1
END SUB

SUB fill (b() AS LONG)
    b(LBOUND(b)) = 1
END SUB

SUB names (b() AS STRING)
    b(0) = "n"
END SUB
