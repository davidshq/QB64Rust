$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (32_sub_argument)
DIM a(2) AS LONG
a(0) = 4
s a()
SYSTEM
SUB s (v AS LONG)
    PRINT v
END SUB
