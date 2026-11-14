$CONSOLE:ONLY
ON ERROR GOTO trap
DIM w(1 TO 64) AS _BYTE, w1(1 TO 1) AS _BYTE
FOR i = 1 TO 64: w(i) = 127 * SIN(i / 64 * 6.2832): NEXT
_WAVE 0, w()
SOUND 440, 1, 0, , 10
PRINT "A wave of 64 samples, ERR is"; ERR
_WAVE 0, w1()
_WAVE 4, w()
END
trap:
PRINT "error"; ERR
RESUME NEXT
