' TEST: typed
$CONSOLE
$SCREENHIDE
' The functions _DEST, _SOURCE and _CONSOLE in the typed tree (m2-console-screenhide task 3.1; measured in
' verification\v26_b_dest_switch, v26_x73 and from the C++): each is called by its bare name and is a LONG, in a
' store, in an expression, as an argument of a built-in and of a procedure, in a condition
DIM d AS LONG, i AS INTEGER, q AS _INTEGER64, f AS DOUBLE, s AS STRING
d = _DEST
d = _SOURCE
d = _CONSOLE
i = _DEST
q = _SOURCE
f = _CONSOLE
d = _DEST + _SOURCE
f = _CONSOLE / 2
d = -_DEST
s = STR$(_DEST) + HEX$(_CONSOLE)
PRINT _DEST; _SOURCE; _CONSOLE
IF _DEST = _CONSOLE THEN d = 1
show _SOURCE
SELECT CASE _DEST
    CASE _CONSOLE
        d = 2
END SELECT

SUB show (h AS LONG)
    PRINT h; _DEST
END SUB
