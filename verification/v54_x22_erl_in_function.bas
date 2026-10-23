$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (22_erl_in_function)
ON ERROR GOTO handler
x = f
PRINT "ERL after f"; ERL
SYSTEM
handler:
PRINT "  ERL"; ERL
RESUME NEXT
FUNCTION f
    70 ERROR 5
    f = 1
END FUNCTION
