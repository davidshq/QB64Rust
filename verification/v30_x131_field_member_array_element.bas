$CONSOLE:ONLY
' Verification (step 4 of M3, first task group of m3-console-files): does the old compiler accept this,
' and what does it do? (131_field_member_array_element)
ON ERROR GOTO h
TYPE t
    s AS STRING * 4
    v AS STRING
    n AS LONG
    m(2) AS LONG
END TYPE
DIM a AS STRING, b AS STRING, x AS STRING * 4, n AS INTEGER, r AS t, r2 AS t, e(3) AS STRING
DIM l AS LONG, fx(3) AS STRING * 4, na(3) AS LONG, d AS DOUBLE
CONST K = "kk"
OPEN "v30_x131.tmp" FOR RANDOM AS #1 LEN = 8
FIELD #1, 4 AS r.m(1)
CLOSE
KILL "v30_x131.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
