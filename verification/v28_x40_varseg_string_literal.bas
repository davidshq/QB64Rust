$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): one form, on the last line, as code that is not reached.
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, t AS STRING, u AS _UNSIGNED LONG
DIM a(100) AS INTEGER, b(1 TO 10, 1 TO 20) AS LONG, ts(10) AS STRING, fs(10) AS STRING * 4
PRINT "ok"
SYSTEM
l = VARSEG("a")
