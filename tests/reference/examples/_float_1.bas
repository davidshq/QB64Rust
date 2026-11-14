$CONSOLE:ONLY
DIM f AS _FLOAT
f = 2.5
PRINT "f ="; f; "takes"; LEN(f); "bytes"
f = 1 / 3: PRINT "1 / 3 stored:"; f
g## = 0.1: PRINT "With the suffix ##:"; g##
END
