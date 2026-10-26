' TEST: debug-info
$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' The debug symbol file's user types (m5-debug-build-symbols task 2.2, spec compiler/debug-info "A type with a
' fixed string"): a fixed string, a STRING member, a member array, a _DYNAMIC member, and types nested in a type,
' at the emitter's offsets (packed; aligned to 8 for a type that owns a _DYNAMIC member).
TYPE pt
    x AS INTEGER
    nm AS STRING * 4
END TYPE
TYPE rec
    p AS pt
    label AS STRING
    v(1 TO 3) AS LONG
END TYPE
TYPE owner
    tag AS _BYTE
    r AS rec
    _DYNAMIC n(1 TO 4) AS LONG
END TYPE
DIM one AS pt
DIM many(2) AS rec
DIM o AS owner
one.x = 1
many(1).label = "a"
o.n(2) = 3
PRINT one.x; many(1).label; o.n(2)
