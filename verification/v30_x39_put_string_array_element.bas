$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (39_put_string_array_element)
ON ERROR GOTO h
OPEN "v30_x39_put_string_array_element.tmp" FOR BINARY AS #1
DIM sa(1 TO 2) AS STRING, v AS STRING
sa(1) = "ab": sa(2) = "cde"
PUT #1, 1, sa(2)
PRINT LOF(1)
v = "??": GET #1, 1, v: PRINT v
GET #1, 2, sa(1): PRINT sa(1)
CLOSE
KILL "v30_x39_put_string_array_element.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
