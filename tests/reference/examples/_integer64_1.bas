$CONSOLE:ONLY
DIM total AS _INTEGER64
total = 3000000000
PRINT "total ="; total; "takes"; LEN(total); "bytes"
PRINT "total * 3 ="; total * 3
x&& = 9223372036854775807
PRINT "The largest value plus 1 wraps to"; x&& + 1
END
