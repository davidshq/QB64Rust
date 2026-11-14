$CONSOLE:ONLY
DIM fixed(3) AS LONG
fixed(2) = 9
ERASE fixed
PRINT "A static array keeps its bounds:"; UBOUND(fixed); "and is cleared:"; fixed(2)
REDIM grow(5) AS LONG
ERASE grow
ON ERROR GOTO noarray
PRINT UBOUND(grow)
REDIM grow(2) AS LONG
PRINT "Given bounds again:"; UBOUND(grow)
END

noarray:
PRINT "Error"; ERR; ": the dynamic array has no elements after ERASE"
RESUME NEXT
