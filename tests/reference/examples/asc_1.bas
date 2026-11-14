$CONSOLE:ONLY
PRINT ASC("A"); ASC("abc", 2)
s$ = "a/b"
ASC(s$, 2) = 92
PRINT s$
ON ERROR GOTO handler
n = ASC("")
END

handler:
PRINT "ASC of an empty string: error"; ERR
RESUME NEXT
