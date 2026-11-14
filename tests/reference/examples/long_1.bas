$CONSOLE:ONLY
DIM population AS LONG
population = 2000000
PRINT "population ="; population; "takes"; LEN(population); "bytes"
big& = 2147483647
PRINT "2147483647 + 1 wraps to"; big& + 1
END
