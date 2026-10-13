$CONSOLE:ONLY
' Verification (step 4 of M3, by the review of m3-console-files): does the old compiler accept this, and
' what does it do? (173_field_function_name)
ON ERROR GOTO h
DIM x AS LONG
OPEN "v30_x173_field_function_name.tmp" FOR RANDOM AS #1 LEN = 8
FIELD #1, 4 AS g$()
CLOSE
KILL "v30_x173_field_function_name.tmp"
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
