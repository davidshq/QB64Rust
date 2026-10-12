$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): COMMON SHARED with a block name before a variable.
COMMON SHARED /blk/ b1 AS LONG
b1 = 1: PRINT b1
SYSTEM
