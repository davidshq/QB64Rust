$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 7, READ into a whole STRING array and a whole LONG array that
' are erased, under a handler that prints ERR and resumes next; the PRINTs after show whether the text read was
' stored where every unallocated array's data points.
ON ERROR GOTO h
DATA "xyz",42
REDIM s(2) AS STRING
ERASE s
READ s()
PRINT "1. after READ s()"
REDIM n(2) AS LONG
ERASE n
READ n()
PRINT "2. after READ n()"
PRINT "3. end"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
