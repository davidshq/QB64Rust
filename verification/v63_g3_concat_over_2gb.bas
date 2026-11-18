$CONSOLE:ONLY
' Verification (DIVERGENCES-QB64PE.md D-067): a concatenation longer than a string can be, 2,147,483,647 bytes,
' under a handler. a$ + a$ with a$ of 1.2 GB adds the two lengths past that.
ON ERROR GOTO trap
a$ = SPACE$(1200000000)
b$ = "x"
b$ = a$ + a$
PRINT "after:"; LEN(a$); LEN(b$)
c$ = a$ + "y"
PRINT "c:"; LEN(c$)
a$ = "": c$ = ""
PRINT "done"
SYSTEM
trap:
PRINT "error"; ERR
RESUME NEXT
