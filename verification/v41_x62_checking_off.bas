$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (62_checking_off)
DIM a(1 TO 5) AS LONG, b(0 TO 9) AS LONG
$CHECKING:OFF
_ARRAYCOPY a(1 TO 9) TO b(0)
$CHECKING:ON
PRINT "ok"
SYSTEM
