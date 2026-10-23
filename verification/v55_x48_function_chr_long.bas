$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x48_function_chr_long)
PRINT "ok"
SYSTEM
FUNCTION chr& (a)
    chr& = a
END FUNCTION
