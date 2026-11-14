$CONSOLE:ONLY
READ a, b$, c
PRINT a; b$; c
ON ERROR GOTO handler
READ d
END

handler:
PRINT "No item left: error"; ERR
RESUME NEXT
DATA 1, two, 3
