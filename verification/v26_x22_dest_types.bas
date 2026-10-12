$CONSOLE
$SCREENHIDE
' Verification (step 11): the handle from variables of several types.
DIM b AS _BYTE, i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE
_DEST _CONSOLE
c& = _CONSOLE
b = c&: i = c&: l = c&: q = c&: s = c&: d = c&
_DEST 0: _DEST b: PRINT "byte"
_DEST 0: _DEST i: PRINT "integer"
_DEST 0: _DEST l: PRINT "long"
_DEST 0: _DEST q: PRINT "integer64"
_DEST 0: _DEST s: PRINT "single"
_DEST 0: _DEST d: PRINT "double"
SYSTEM
