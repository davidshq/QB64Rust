$CONSOLE:ONLY
' Verification (step 4 of M3, by the review of m3-console-files): does the old compiler accept this, and
' what does it do? (177_put_undeclared_array)
ON ERROR GOTO h
DIM x AS LONG
OPEN "v30_x177_put_undeclared_array.tmp" FOR BINARY AS #1
PUT #1, , und()
CLOSE
KILL "v30_x177_put_undeclared_array.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
FUNCTION f
    f = 1
END FUNCTION
FUNCTION g$
    g$ = "a"
END FUNCTION
