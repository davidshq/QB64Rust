$CONSOLE:ONLY
' Verification (m2-control-flow-slice, design Context, probe 1): errors in block headers. The handler prints ERR
' and resumes next; it makes the failing call succeed every third time (k = 65), so every loop ends.
DIM k AS LONG
ON ERROR GOTO h
k = -1
PRINT "single-line IF"
IF CHR$(k) = "a" THEN PRINT "  then" ELSE PRINT "  else"
k = -1
PRINT "block IF"
IF CHR$(k) = "a" THEN
    PRINT "  then"
ELSE
    PRINT "  else"
END IF
k = -1: n = 0
PRINT "WHILE"
WHILE LEN(CHR$(k)) = 1 AND n < 2
    n = n + 1: PRINT "  body"; n
WEND
k = -1: n = 0
PRINT "DO WHILE"
DO WHILE LEN(CHR$(k)) = 1 AND n < 2
    n = n + 1: PRINT "  body"; n
LOOP
k = -1: n = 0
PRINT "LOOP UNTIL"
DO
    n = n + 1: PRINT "  body"; n
LOOP UNTIL CHR$(k) = "a" OR n >= 3
PRINT "  after"; n
k = -1
PRINT "FOR"
FOR i = 1 TO LEN(CHR$(k)) + 2
    PRINT "  body"; i
NEXT
PRINT "  after"; i
SYSTEM

h:
errs = errs + 1
PRINT "  handler"; ERR
IF errs MOD 3 = 0 THEN k = 65
IF errs > 30 THEN PRINT "too many errors": SYSTEM
RESUME NEXT
