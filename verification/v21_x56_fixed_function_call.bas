$CONSOLE:ONLY
' Rejection (review of m2-numeric-types): CALL of a FUNCTION fs$5.
CALL fs$5("a")
PRINT "ok"
SYSTEM

FUNCTION fs$5 (x AS STRING)
fs$5 = x
END FUNCTION
