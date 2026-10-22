$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (21_on_error_number_in_sub)
ON ERROR GOTO 10
ERROR 5
SYSTEM
SUB s
    10 PRINT "in s"
END SUB
