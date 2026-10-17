$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: NOT as the argument
CONST z = CHR$(NOT -66)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
