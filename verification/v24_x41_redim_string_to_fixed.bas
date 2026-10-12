$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM of a STRING array as STRING * 4.
REDIM x(5) AS STRING
REDIM x(5) AS STRING * 4
PRINT LEN(x(1))
SYSTEM
