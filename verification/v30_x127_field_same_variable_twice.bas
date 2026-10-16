$CONSOLE:ONLY
' Verification (step 4 of M3, first task group of m3-console-files): does the old compiler accept this,
' and what does it do? (127_field_same_variable_twice)
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
OPEN "v30_x127.tmp" FOR RANDOM AS #1 LEN = 8
FIELD #1, 3 AS a, 5 AS a
LSET a = "12345678": PRINT "["; a; "]"; LEN(a)
PUT #1, 1: FIELD #1, 8 AS b: GET #1, 1: FOR l = 1 TO 8: PRINT ASC(b, l);: NEXT: PRINT
CLOSE
KILL "v30_x127.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
