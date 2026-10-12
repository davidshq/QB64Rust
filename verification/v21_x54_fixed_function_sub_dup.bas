$CONSOLE:ONLY
' Rejection (review of m2-numeric-types): SUB fs$5 after FUNCTION fs$5.
PRINT fs$5("a")
SYSTEM

FUNCTION fs$5 (x AS STRING)
fs$5 = x
END FUNCTION
SUB fs$5
END SUB
