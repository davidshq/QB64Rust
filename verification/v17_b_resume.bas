$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): RESUME (retry) after an error in each kind of block header. The
' handler makes the failing call succeed (k = 65, CHR$(65) = "A") and retries; it gives up after 30 errors.
DIM k AS LONG
ON ERROR GOTO h
PRINT "IF"
k = -1
IF CHR$(k) = "A" THEN PRINT "  then" ELSE PRINT "  else"
PRINT "ELSEIF"
k = -1
IF k = 5 THEN
    PRINT "  if"
ELSEIF CHR$(k) = "A" THEN
    PRINT "  elseif"
ELSE
    PRINT "  else"
END IF
PRINT "WHILE"
k = -1: n = 0
WHILE CHR$(k) = "A" AND n < 2
    n = n + 1: PRINT "  body"; n
WEND
PRINT "DO WHILE"
k = -1: n = 0
DO WHILE CHR$(k) = "A" AND n < 2
    n = n + 1: PRINT "  body"; n
LOOP
PRINT "DO UNTIL"
k = -1: n = 0
DO UNTIL CHR$(k) <> "A" OR n >= 2
    n = n + 1: PRINT "  body"; n
LOOP
PRINT "LOOP WHILE"
k = -1: n = 0
DO
    n = n + 1: PRINT "  body"; n
LOOP WHILE CHR$(k) = "A" AND n < 3
PRINT "LOOP UNTIL"
k = -1: n = 0
DO
    n = n + 1: PRINT "  body"; n
LOOP UNTIL CHR$(k) <> "A" OR n >= 3
PRINT "FOR start"
k = -1
FOR i = ASC(CHR$(k)) - 63 TO 4
    PRINT "  body"; i
NEXT
PRINT "  after"; i
PRINT "FOR limit"
k = -1
FOR i = 1 TO ASC(CHR$(k)) - 62
    PRINT "  body"; i
NEXT
PRINT "  after"; i
PRINT "FOR step"
k = -1
FOR i = 1 TO 5 STEP ASC(CHR$(k)) - 63
    PRINT "  body"; i
NEXT
PRINT "  after"; i
PRINT "end"
SYSTEM

h:
errs = errs + 1
PRINT "  handler"; ERR
IF errs > 30 THEN PRINT "too many errors": SYSTEM
k = 65
RESUME
