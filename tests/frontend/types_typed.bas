' TEST: typed
$CONSOLE:ONLY
' User types (m2-arrays-and-types tasks 4.1, 4.2): a type used above its block, nested types, members read and
' written, a member with its own suffix, a dotted plain name, a dotted name before the DIM of its first part
' (file order), arrays of a type, members passed by reference, a TYPE variable local to a SUB
DIM early AS outer
TYPE inner
    v AS LONG
    w AS INTEGER
END TYPE
TYPE outer
    i AS inner
    d AS DOUBLE
END TYPE
early.i.v = 1
early.d = early.i.w + 2.5
early.i.v& = early.i.v& * 2
plain.name = 3
later.v = 4
DIM later AS inner
later.v = later.v + 1
DIM arr(2) AS outer
arr(1).i.w = arr(2).i.v
bump early.i.v
bump arr(0).i.v
show
SUB bump (n AS LONG)
    n = n + 1
END SUB
SUB show
    DIM p AS inner
    p.w = 1
END SUB
