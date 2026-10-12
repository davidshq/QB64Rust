$CONSOLE:ONLY
' Rejection (review of m2-numeric-types): CONST fs$5 beside FUNCTION fs$5.
CONST fs$5 = "a"
PRINT fs$5("a")
SYSTEM

FUNCTION fs$5 (x AS STRING)
fs$5 = x
END FUNCTION
