' TEST: ir
$CONSOLE:ONLY
' Whole values in the IR (m2-defxxx-common-arrays task 8.2, spec compiler/pipeline): q = p is one CopyWhole from
' a place to a place, with no guard; an argument for a parameter of a user type is the place, by reference.
TYPE pt
    x AS LONG
    n AS STRING
END TYPE
TYPE box
    a AS pt
END TYPE
DIM p AS pt, q AS pt, a(3) AS pt, bx AS box, i AS LONG
q = p
a(i) = q
q = a(i + 1)
bx.a = a(2)
move q
move a(i)
move bx.a
SYSTEM

SUB move (p AS pt)
    DIM l AS pt
    l = p
    p.x = l.x + 1
END SUB
