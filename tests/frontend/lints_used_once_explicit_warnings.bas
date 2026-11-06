' TEST: warnings
' With `OPTION _EXPLICIT` every variable is declared, so `used-once` never fires.
OPTION _EXPLICIT
$CONSOLE:ONLY
DIM total AS INTEGER
total = 5
PRINT total
