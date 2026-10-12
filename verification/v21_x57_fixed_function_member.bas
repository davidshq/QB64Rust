$CONSOLE:ONLY
' Rejection (review of m2-numeric-types): a TYPE member fs$5 beside FUNCTION fs$5.
TYPE t
  fs$5 AS STRING
END TYPE
PRINT fs$5("a")
SYSTEM

FUNCTION fs$5 (x AS STRING)
fs$5 = x
END FUNCTION
