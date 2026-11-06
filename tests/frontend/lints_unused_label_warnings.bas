' TEST: warnings
' `unused-label` (spec `compiler/lints`, "Unused label"): only `again` and `spare` are reported.
$CONSOLE:ONLY
ON ERROR GOTO handler
ON TIMER(1) GOSUB tick
RESTORE numbers
READ n
again:
PRINT n
GOSUB show
ON n GOTO first, second
first:
second:
IF n = 7 THEN GOTO 20
10 PRINT "ten"
20 PRINT "twenty"
30 IF n THEN 40
40 ERROR 5
GOTO finish
handler:
RESUME after
after:
RETURN
show:
PRINT "show"
RETURN
tick:
RETURN
numbers:
DATA 7
finish:
Inner
SUB Inner
spare:
END SUB
