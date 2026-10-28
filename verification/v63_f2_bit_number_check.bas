$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 17, the program of the fix bit-number-check: bit numbers in
' variables inside and outside 0 to 63 under a handler. With the fixes off, QB64pe's values; its fixes-on output is
' tests\verification\fixed\v63_f2_bit_number_check.out.txt.
ON ERROR GOTO h
DIM n AS LONG
n = 63: PRINT "63:"; _SETBIT(0, n)
n = 64: PRINT "setbit 64:"; _SETBIT(5, n)
n = -1: PRINT "readbit -1:"; _READBIT(1, n)
n = 70: PRINT "resetbit 70:"; _RESETBIT(127, n)
n = -3: PRINT "togglebit -3:"; _TOGGLEBIT(5, n)
n = 0: PRINT "0:"; _READBIT(1, n)
n = 64: PRINT "shl 64:"; _SHL(1, n)
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
