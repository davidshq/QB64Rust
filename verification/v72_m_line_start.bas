$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): a built-in statement's name and a colon, a label, and a SUB's name and a colon.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
CLS: PRINT "start": n = 1
top: x = 9: b = 9
PRINT x; b; n
DrawBoard: PRINT "Ready": x = 1
PRINT x
SYSTEM
SUB DrawBoard
PRINT "board"
END SUB
