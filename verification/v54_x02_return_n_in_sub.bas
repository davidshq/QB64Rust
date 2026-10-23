$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (02_return_n_in_sub)
s
PRINT "back"
SYSTEM
SUB s
    GOSUB 10
    PRINT "skipped"
    20 PRINT "at 20"
    EXIT SUB
    10 PRINT "at 10"
    RETURN 20
END SUB
