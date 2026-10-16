$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (43_suffix_strings)
DIM a$(2), b$(2)
b$(0) = "x": b$(1) = "y"
a$() = b$()
PRINT a$(0); a$(1)
PRINT "ok"
SYSTEM
