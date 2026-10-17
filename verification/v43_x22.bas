$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: an argument of 2^64
CONST z = CHR$(18446744073709551616)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
