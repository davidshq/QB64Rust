$CONSOLE:ONLY
' Verification (OpenSpec change eval-order-left-to-right, task 2.1): the order in which LINE, PSET and CIRCLE
' evaluate FUNCTION coordinates and colors. They draw on an image, so each FUNCTION adds its name to o$, which is
' printed on the console after the statement.
DIM SHARED o AS STRING, img AS LONG, p AS LONG
img = _NEWIMAGE(20, 20, 32)
_DEST img
PSET (f(1), f(2)), f(3)
show "PSET"
LINE (f(1), f(2))-(f(3), f(4)), f(5)
show "LINE"
LINE (f(1), f(2))-(f(3), f(4)), f(5), BF
show "LINE BF"
CIRCLE (f(10), f(10)), f(5), f(7)
show "CIRCLE"
_SOURCE img
p = POINT(f(1), f(2))
show "POINT"
SYSTEM

SUB show (what AS STRING)
    _DEST _CONSOLE
    PRINT what; ":"; o
    o = ""
    _DEST img
END SUB

FUNCTION f& (k AS LONG)
    o = o + " f" + LTRIM$(STR$(k))
    f& = k
END FUNCTION
