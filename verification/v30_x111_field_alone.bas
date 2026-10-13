$CONSOLE:ONLY
' Verification (step 4 of M3, first task group of m3-console-files): does the old compiler accept this,
' and what does it do? (111_field_alone)
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
OPEN "v30_x111.tmp" FOR RANDOM AS #1 LEN = 8
FIELD
CLOSE
KILL "v30_x111.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
