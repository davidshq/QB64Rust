$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM of a STRING * 3 array as STRING * 5.
REDIM x(5) AS STRING * 3
REDIM x(5) AS STRING * 5
PRINT LEN(x(1))
SYSTEM
