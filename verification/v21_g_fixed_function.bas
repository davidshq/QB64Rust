$CONSOLE:ONLY
' Verification (m2-numeric-types, task 8.2): a FUNCTION named f$n called and assigned by its other spellings
' (fs$5 with its length, fs$ and fs without), a bare f$n FUNCTION never assigned (n NUL bytes), and a FUNCTION result
' used in an expression. fs$4("a") is "Name already in use" (v21_x49).
PRINT "["; fs$5("ab"); "]"; LEN(fs$5("ab"))
PRINT "["; fs$("abcdefgh"); "]"
PRINT "["; fs("q") + "!"; "]"
PRINT "["; fz$3; "]"; LEN(fz$3); ASC(fz$3)
PRINT "["; gs$4("xy"); "]"
SYSTEM

FUNCTION fs$5 (x AS STRING)
fs$5 = x
END FUNCTION

FUNCTION fz$3
END FUNCTION

FUNCTION gs$4 (x AS STRING)
gs = x + "1234"
END FUNCTION
