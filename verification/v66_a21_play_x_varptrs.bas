$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-sound row on VARPTR$ in PLAY and DRAW): PLAY's X command with VARPTR$ of a string variable: a valid string, one with a letter PLAY rejects, and that letter alone.
ON ERROR GOTO h
p$ = "MB L64 C"
PLAY "X" + VARPTR$(p$)
PRINT "valid:"; e
e = 0: q$ = "Q"
PLAY "X" + VARPTR$(q$)
PRINT "Q inside X:"; e
e = 0
PLAY "Q"
PRINT "Q alone:"; e
SYSTEM
h:
e = ERR
RESUME NEXT
