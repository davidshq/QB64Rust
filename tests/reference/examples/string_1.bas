$CONSOLE:ONLY
DIM name1 AS STRING
name1 = "Wellington"
PRINT name1; " has"; LEN(name1); "characters"
DIM code AS STRING * 4
code = "ab"
PRINT "["; code; "]"; LEN(code)
code = "abcdef"
PRINT "["; code; "]"
title$ = "Duke"
PRINT title$ + " of " + name1
END
