$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x72_whole_in_sub_twice)
ON ERROR GOTO h
s: s
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
SUB s
    PRINT LEN(u()); u()
END SUB
