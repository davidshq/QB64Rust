' TEST: typed
$CONSOLE:ONLY
' Names, declarations and arrays (m4-names-arrays; verification\v48_*). A scalar and an array of one name, one typed
' by AS (design D1, D2): the plain scalar is of the default type; between two arrays the one typed by AS keeps the
' plain name, but for a local array beside a shared one and an array beside a STATIC list nothing has reached.
' REDIM name [AS type] without parentheses in a SUB or FUNCTION (D3): a scalar, as DIM declares it. TYPE blocks
' anywhere (D4): one of a SUB used above it in the main module and as another SUB's parameter type, one of an IF
' block, one of a FUNCTION used in the main module, OPTION BASE 1 above a block in a SUB.
DIM SHARED sh(3) AS INTEGER
DIM a(3) AS INTEGER
a = 1.5: a(1) = 7
PRINT a; a(1); LEN(a)
DIM b AS STRING
DIM b(2)
b(1) = 1.25
DIM c(2) AS DOUBLE
DIM c(3)
c(1) = c(1) + 1
c!(1) = 2
DIM d AS STRING
DIM d(3) AS LONG
d = "dd": d(1) = 5
REDIM e(2) AS DOUBLE
REDIM e(3)
e(1) = 0.5
plain_in_sub
local_beside_shared
beside_list
redim_list "abc"
redim_static
DIM q AS pt
q.x = 3
use_pt q
DIM r AS inif
r.v = 4
IF 0 THEN
    TYPE inif
        v AS _INTEGER64
    END TYPE
END IF
DIM fr AS infn
fr.z = 2.5
DIM ob AS based
ob.m(1) = 7
PRINT LBOUND(ob.m); LEN(ob); getx&(q)
SUB plain_in_sub
    DIM a(3) AS INTEGER
    a = 2.5: a(1) = 9
    sh = 3.5: sh(1) = 4
END SUB
SUB local_beside_shared
    DIM sh(3)
    sh(1) = 0.5
END SUB
SUB beside_list
    STATIC l() AS DOUBLE
    DIM l(3)
    l(1) = l(1) + 1
END SUB
SUB redim_list (s AS STRING)
    REDIM v(LEN(s)) AS LONG, i AS LONG
    FOR i = 1 TO UBOUND(v): v(i) = i * 100000: NEXT
END SUB
SUB redim_static STATIC
    REDIM i
    i = i + 0.5
END SUB
SUB holds_pt
    TYPE pt
        x AS LONG
        y AS LONG
    END TYPE
    DIM p AS pt
    p.y = 5
END SUB
SUB use_pt (p AS pt)
    PRINT p.x
END SUB
FUNCTION getx& (p AS pt)
    TYPE infn
        z AS DOUBLE
    END TYPE
    getx& = p.x * 2
END FUNCTION
SUB holds_based
    OPTION BASE 1
    TYPE based
        m(3) AS LONG
    END TYPE
END SUB
FUNCTION redim_fn& (n)
    REDIM r AS INTEGER
    r = r + n
    redim_fn& = r
END FUNCTION
