$CONSOLE:ONLY
DIM a AS LONG
DIM o AS _OFFSET
o = _OFFSET(a)
PRINT "An _OFFSET takes"; LEN(o); "bytes"
PRINT "The address of a is not 0:"; o <> 0
END
