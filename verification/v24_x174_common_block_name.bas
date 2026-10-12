$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): COMMON with a block name before a variable.
COMMON /blk/ b1 AS LONG
b1 = 1: PRINT b1
SYSTEM
