$CONSOLE:ONLY
' A dynamic array with fewer than no element (spec language/arrays-and-types, "Dynamic arrays", scenario "Fewer
' than no element"): an upper bound two or more below the lower bound is "Out of memory", which ends the program
' and which no handler receives; the line after the DIM does not run.
ON ERROR GOTO h
DIM n AS LONG
n = -3
PRINT "DIM d(n) with n = -3:";
DIM d(n) AS LONG
PRINT " then UBOUND(d):";
PRINT UBOUND(d)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
