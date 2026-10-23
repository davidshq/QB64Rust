$CONSOLE:ONLY
' Verification (m4-handlers-logging-checking, task 1.2): does the old compiler accept this? (x12_checking_in_if)
x = 1
IF x THEN
$CHECKING:OFF
PRINT "ok"
$CHECKING:ON
END IF
SYSTEM
