$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D9): IF c THEN <line number> with a condition that raises, with RESUME NEXT.
' The placeholder (ASC("") gives 0) makes the condition false; v17_b_elseif case 9 measured the IF c GOTO form.
ON ERROR GOTO h
PRINT "case 1: IF c THEN 100, placeholder makes it false"
IF ASC("") = 7 THEN 100
PRINT "  not jumped"
GOTO 200
100 PRINT "  jumped"
200 PRINT "case 2: IF c THEN 300 ELSE 400, placeholder makes it false"
IF ASC("") = 7 THEN 300 ELSE 400
PRINT "  fell through"
GOTO 500
300 PRINT "  then"
GOTO 500
400 PRINT "  else"
500 PRINT "end"
SYSTEM

h:
PRINT "  handler"; ERR
RESUME NEXT
