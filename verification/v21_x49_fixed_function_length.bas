$CONSOLE:ONLY
' Rejection (m2-numeric-types, task 8.2): a FUNCTION fs$5 called with another length.
PRINT fs$4("a")
SYSTEM

FUNCTION fs$5 (x AS STRING)
fs$5 = x
END FUNCTION
