$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): an integer-valued negative float and a negative zero in CONST.
CONST a = -4 / 2, b = 0 * -1, c = -0.5 * 2
PRINT a; b; c; a * 1000000000
SYSTEM
