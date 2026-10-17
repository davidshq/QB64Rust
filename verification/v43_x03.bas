$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: a string argument
CONST z = CHR$("A")
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
