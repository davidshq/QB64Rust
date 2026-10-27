$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 7, a whole array with no storage used as a value: an erased
' LONG array, an erased STRING array, and a dynamic array a GOTO kept from being allocated, under a handler that
' prints ERR and resumes next.
'$DYNAMIC
ON ERROR GOTO h
REDIM a(2) AS LONG
a(0) = 5
ERASE a
PRINT "1."; a()
REDIM s(2) AS STRING
s(0) = "abc"
ERASE s
PRINT "2. ["; s(); "]"
GOTO skip
DIM never(3) AS LONG
skip:
PRINT "3."; never()
PRINT "4."; a() + 1
PRINT "5. end"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
