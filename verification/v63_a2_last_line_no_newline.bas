$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 2, a last line without a newline (v63_a2_last_line_no_newline.stdin
' holds "abc" and no newline): whether LINE INPUT takes it, and what the INPUT after it does.
ON ERROR GOTO h
DIM s AS STRING, n AS LONG
LINE INPUT s
PRINT "<"; s; ">"
INPUT n
PRINT "<"; n; ">"
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]"
RESUME NEXT
