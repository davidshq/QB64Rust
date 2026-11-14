$CONSOLE:ONLY
PRINT _CEIL(2.1); _CEIL(2); _CEIL(-2.9)
items = 23: perPage = 10
PRINT "Pages needed:"; _CEIL(items / perPage)
END
