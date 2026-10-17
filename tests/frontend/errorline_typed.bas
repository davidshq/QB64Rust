' TEST: typed
$CONSOLE:ONLY
' The error functions (m3-errorline-chr-const; verification\v43_a_errorline, v43_x43): _ERRORLINE and
' _INCLERRORLINE are LONG, _INCLERRORFILE$ a string, _ERRORMESSAGE$ a string with its code optional, a LONG
' slot (a DOUBLE is rounded into it).
DIM n AS LONG, s AS STRING, d AS DOUBLE
n = _ERRORLINE
n = _INCLERRORLINE
s = _INCLERRORFILE$
s = _ERRORMESSAGE$
s = _ERRORMESSAGE$(n)
s = _ERRORMESSAGE$(d)
PRINT _ERRORLINE + 1; _ERRORLINE / 2
SYSTEM
