$CONSOLE:ONLY
' Errors in block headers (spec language/error-handling, "Errors in block headers"; spec compiler/pipeline, the
' pending-error rule): each header kind with RESUME NEXT, then with RESUME, then untrapped (run with
' QB64PE_NOPROMPT=continue, s19_header_errors.noprompt). CHR$(-1) raises error 5 and gives "" as its placeholder.
' Also a store made with a placeholder value, a SUB called with a raising argument, and an ELSEIF that raises
' (tested with the placeholder; when false, the error is serviced at the next statement that runs).
' Caps: the RESUME NEXT handler makes the call succeed every third error of a case (k = 65), every handler stops
' the program after 30 errors of a case, the loops of the RESUME NEXT and untrapped parts leave after a few passes,
' and those of the RESUME part are bounded by their counters. A FOR case that should show its stored limit or
' step starts with the variable at 0, since the body sees its old value.
DIM k AS LONG
ON ERROR GOTO hnext
PRINT "RESUME NEXT"
PRINT "single-line IF"
k = -1: errs = 0
IF CHR$(k) = "a" THEN PRINT "  then" ELSE PRINT "  else"
PRINT "block IF"
k = -1: errs = 0
IF CHR$(k) = "a" THEN
    PRINT "  then"
ELSE
    PRINT "  else"
END IF
PRINT "ELSEIF, the placeholder makes it false"
k = -1: errs = 0: x = 1
IF x = 2 THEN
    PRINT "  if"
ELSEIF CHR$(k) = "a" THEN
    PRINT "  elseif"
ELSE
    PRINT "  else"
    PRINT "  else 2"
END IF
PRINT "ELSEIF, the placeholder makes it true"
k = -1: errs = 0
IF x = 2 THEN
    PRINT "  if"
ELSEIF CHR$(k) = "" THEN
    PRINT "  elseif"
ELSE
    PRINT "  else"
END IF
PRINT "two ELSEIFs, the first raises and is false"
k = -1: errs = 0
IF x = 2 THEN
    PRINT "  if"
ELSEIF CHR$(k) = "a" THEN
    PRINT "  elseif 1"
ELSEIF x = 1 THEN
    PRINT "  elseif 2"
ELSE
    PRINT "  else"
END IF
PRINT "IF ... GOTO"
k = -1: errs = 0
IF CHR$(k) = "a" GOTO jumped
PRINT "  not jumped"
jumped:
PRINT "  after"
PRINT "WHILE"
k = -1: errs = 0: n = 0
WHILE CHR$(k) <> "" AND n < 2
    n = n + 1: PRINT "  body"; n
    IF n > 6 THEN EXIT WHILE
WEND
PRINT "DO WHILE"
k = -1: errs = 0: n = 0
DO WHILE CHR$(k) <> "" AND n < 2
    n = n + 1: PRINT "  body"; n
    IF n > 6 THEN EXIT DO
LOOP
PRINT "DO UNTIL"
k = -1: errs = 0: n = 0
DO UNTIL CHR$(k) = "" OR n >= 2
    n = n + 1: PRINT "  body"; n
    IF n > 6 THEN EXIT DO
LOOP
PRINT "LOOP WHILE"
k = -1: errs = 0: n = 0
DO
    n = n + 1: PRINT "  body"; n
    IF n > 6 THEN EXIT DO
LOOP WHILE CHR$(k) <> "" AND n < 3
PRINT "  after"; n
PRINT "LOOP UNTIL"
k = -1: errs = 0: n = 0
DO
    n = n + 1: PRINT "  body"; n
    IF n > 6 THEN EXIT DO
LOOP UNTIL CHR$(k) = "a" OR n >= 3
PRINT "  after"; n
PRINT "FOR limit"
k = -1: errs = 0: n = 0
FOR i = 1 TO INSTR("xyz", CHR$(k)) + 2
    n = n + 1: PRINT "  body"; i
    IF n > 6 THEN EXIT FOR
NEXT
PRINT "  after"; i
PRINT "FOR start"
k = -1: errs = 0: n = 0: i = 7
FOR i = INSTR("xyA", CHR$(k)) TO 4
    n = n + 1: PRINT "  body"; i
    IF n > 6 THEN EXIT FOR
NEXT
PRINT "  after"; i
PRINT "FOR step"
k = -1: errs = 0: n = 0: i = 0
FOR i = 1 TO 3 STEP INSTR("xA", CHR$(k))
    n = n + 1: PRINT "  body"; i
    IF n > 6 THEN EXIT FOR
NEXT
PRINT "  after"; i
PRINT "a store made with a placeholder value"
k = -1: errs = 0
s$ = "a": s$ = CHR$(k)
PRINT "  ["; s$; "]"
s$ = "a": s$ = "<" + CHR$(k) + ">"
PRINT "  ["; s$; "]"
y = 5: y = INSTR("abc", CHR$(k))
PRINT "  y"; y
PRINT "a SUB called with a raising argument"
k = -1: errs = 0
showstr CHR$(k)
PRINT "  after showstr"

ON ERROR GOTO hretry
PRINT "RESUME"
PRINT "IF"
k = -1: errs = 0
IF CHR$(k) = "A" THEN PRINT "  then" ELSE PRINT "  else"
PRINT "ELSEIF"
k = -1: errs = 0
IF k = 5 THEN
    PRINT "  if"
ELSEIF CHR$(k) = "A" THEN
    PRINT "  elseif"
ELSE
    PRINT "  else"
END IF
PRINT "WHILE"
k = -1: errs = 0: n = 0
WHILE CHR$(k) = "A" AND n < 2
    n = n + 1: PRINT "  body"; n
WEND
PRINT "DO WHILE"
k = -1: errs = 0: n = 0
DO WHILE CHR$(k) = "A" AND n < 2
    n = n + 1: PRINT "  body"; n
LOOP
PRINT "DO UNTIL"
k = -1: errs = 0: n = 0
DO UNTIL CHR$(k) <> "A" OR n >= 2
    n = n + 1: PRINT "  body"; n
LOOP
PRINT "LOOP WHILE"
k = -1: errs = 0: n = 0
DO
    n = n + 1: PRINT "  body"; n
LOOP WHILE CHR$(k) = "A" AND n < 3
PRINT "LOOP UNTIL"
k = -1: errs = 0: n = 0
DO
    n = n + 1: PRINT "  body"; n
LOOP UNTIL CHR$(k) <> "A" OR n >= 3
PRINT "FOR start"
k = -1: errs = 0
FOR i = INSTR("xyA", CHR$(k)) TO 4
    PRINT "  body"; i
NEXT
PRINT "  after"; i
PRINT "FOR limit"
k = -1: errs = 0
FOR i = 1 TO INSTR("xyA", CHR$(k))
    PRINT "  body"; i
NEXT
PRINT "  after"; i
PRINT "FOR step"
k = -1: errs = 0
FOR i = 1 TO 5 STEP INSTR("xA", CHR$(k))
    PRINT "  body"; i
NEXT
PRINT "  after"; i

ON ERROR GOTO 0
PRINT "untrapped"
PRINT "IF"
k = -1
IF CHR$(k) = "A" THEN PRINT "  then" ELSE PRINT "  else"
PRINT "WHILE"
n = 0
WHILE CHR$(k) = "A" AND n < 2
    n = n + 1: PRINT "  body"; n
    IF n > 2 THEN EXIT WHILE
WEND
PRINT "LOOP UNTIL"
n = 0
DO
    n = n + 1: PRINT "  body"; n
    IF n > 6 THEN EXIT DO
LOOP UNTIL CHR$(k) = "A" OR n >= 3
PRINT "FOR limit"
n = 0: i = 0
FOR i = 1 TO INSTR("xyz", CHR$(k)) + 2
    n = n + 1: PRINT "  body"; i
    IF n > 6 THEN EXIT FOR
NEXT
PRINT "  after"; i
PRINT "end"
SYSTEM

hnext:
errs = errs + 1
PRINT "  handler"; ERR
IF errs MOD 3 = 0 THEN k = 65
IF errs > 30 THEN PRINT "too many errors": SYSTEM
RESUME NEXT

hretry:
errs = errs + 1
PRINT "  handler"; ERR
IF errs > 30 THEN PRINT "too many errors": SYSTEM
k = 65
RESUME

SUB showstr (t AS STRING)
    PRINT "  in showstr ["; t; "]"
END SUB
