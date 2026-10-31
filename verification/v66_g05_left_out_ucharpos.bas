$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-window-output row on arguments left out): _UCHARPOS's third argument left out; one form, on the last line, as code that is not reached.
DIM p(2) AS LONG
PRINT "ok"
SYSTEM
x = _UCHARPOS("ab", p(), , 0)
