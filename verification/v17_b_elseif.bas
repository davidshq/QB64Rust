$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): an ELSEIF condition that raises, with RESUME NEXT. The old compiler
' writes `if (e){` for ELSEIF without the error-pending term, so the branch would depend on the placeholder value
' (CHR$(-1) gives "", ASC("") gives 0; v17_a_pending). Each case tells the two readings apart.
DIM k AS LONG
k = -1
ON ERROR GOTO h
x = 1
PRINT "case 1: string, placeholder makes it false"
IF x = 2 THEN
    PRINT "  if"
ELSEIF CHR$(k) = "a" THEN
    PRINT "  elseif"
ELSE
    PRINT "  else"
END IF
PRINT "case 2: string, placeholder makes it true"
IF x = 2 THEN
    PRINT "  if"
ELSEIF CHR$(k) = "" THEN
    PRINT "  elseif"
ELSE
    PRINT "  else"
END IF
PRINT "case 3: numeric, placeholder makes it false"
IF x = 2 THEN
    PRINT "  if"
ELSEIF ASC("") = 7 THEN
    PRINT "  elseif"
ELSE
    PRINT "  else"
END IF
PRINT "case 4: numeric, placeholder makes it true"
IF x = 2 THEN
    PRINT "  if"
ELSEIF ASC("") = 0 THEN
    PRINT "  elseif"
ELSE
    PRINT "  else"
END IF
PRINT "case 5: two ELSEIFs, the first raises and is false by placeholder"
IF x = 2 THEN
    PRINT "  if"
ELSEIF ASC("") = 7 THEN
    PRINT "  elseif 1"
ELSEIF x = 1 THEN
    PRINT "  elseif 2"
ELSE
    PRINT "  else"
END IF
PRINT "case 6: IF condition, placeholder makes it false"
IF ASC("") = 7 THEN
    PRINT "  if"
ELSEIF x = 1 THEN
    PRINT "  elseif"
ELSE
    PRINT "  else"
END IF
PRINT "case 7: single-line IF with ELSE, placeholder makes it false"
IF ASC("") = 7 THEN PRINT "  then" ELSE PRINT "  else"
PRINT "case 8: single-line IF without ELSE, placeholder makes it false"
IF ASC("") = 7 THEN PRINT "  then": PRINT "  then 2"
PRINT "case 9: IF c GOTO, placeholder makes it false"
IF ASC("") = 7 GOTO jumped
PRINT "  not jumped"
GOTO done9
jumped:
PRINT "  jumped"
done9:
PRINT "end"
SYSTEM

h:
PRINT "  handler"; ERR
RESUME NEXT
