$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, review of group 5): below $DYNAMIC, an array used without DIM first
' named by a later target of INPUT and of LINE INPUT's prompt-less form; each runs twice (the lines typed are in
' the .stdin file).
ON ERROR GOTO h
DIM i AS LONG, a AS LONG, b AS LONG
'$DYNAMIC
PRINT "1. INPUT a, ia(1), b:";
FOR i = 1 TO 2
    INPUT a, ia(1), b
NEXT
PRINT a; ia(1); b
PRINT "2. INPUT with a prompt, the first target names it:";
FOR i = 1 TO 2
    INPUT "p"; ib(1), a
NEXT
PRINT ib(1); a
PRINT "3. LINE INPUT into an element:";
FOR i = 1 TO 2
    LINE INPUT ls$(1)
NEXT
PRINT "["; ls$(1); "]"
PRINT "4. what is left to read:";
LINE INPUT r$
PRINT "["; r$; "]"

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
