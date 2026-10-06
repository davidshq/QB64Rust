$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D9): IF c THEN <label> (a name, not a line number) with a raising condition.
ON ERROR GOTO h
PRINT "case 1: IF c THEN label, placeholder makes it false"
IF ASC("") = 7 THEN jumped
PRINT "  not jumped"
GOTO done
jumped:
PRINT "  jumped"
done:
PRINT "end"
SYSTEM

h:
PRINT "  handler"; ERR
RESUME NEXT
