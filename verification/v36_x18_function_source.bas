$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (18_function_source)
DIM a(2) AS LONG
a() = f()
PRINT "ok"
SYSTEM
FUNCTION f& ()
    f& = 1
END FUNCTION
