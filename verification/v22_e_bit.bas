$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Console input"): INPUT into _BIT variables (v22_e_bit.stdin: 1, then
' -1, then 5,3): does the old compiler accept it, and what is stored?
ON ERROR GOTO h
DIM b AS _BIT, b3 AS _BIT * 3, ub3 AS _UNSIGNED _BIT * 3
INPUT b
PRINT "<"; b; ">"
INPUT b
PRINT "<"; b; ">"
INPUT b3, ub3
PRINT "<"; b3; "><"; ub3; ">"
PRINT "done"
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
