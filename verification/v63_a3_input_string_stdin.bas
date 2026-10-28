$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 2, INPUT$(n) with no file number in a $CONSOLE:ONLY program whose
' input is a file (v63_a3_input_string_stdin.stdin holds "abcdef" and a newline): whether it reads the redirected
' input at all, and then what it does at its end.
ON ERROR GOTO h
DIM s AS STRING
s = INPUT$(3)
PRINT "<"; s; ">"
s = INPUT$(10)
PRINT "<"; s; ">"
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]"
RESUME NEXT
