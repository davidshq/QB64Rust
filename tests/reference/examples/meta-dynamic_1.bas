$CONSOLE:ONLY
'$DYNAMIC
DIM big(5) AS LONG
big(5) = 1
REDIM big(10) AS LONG
PRINT "big can be given new bounds:"; UBOUND(big)
END
