$CONSOLE:ONLY
' Verification (m3-window-output, task 3.2): a built-in statement's name and parentheses where the old
' compiler does not read an assignment (something stands between the parenthesis and the equals sign), and
' WIDTH, which it takes as a variable and as an array; as code that is not reached.
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, t AS STRING, k AS STRING
PRINT "ok"
SYSTEM
CLS (1) + 2 = 5
CLS ((1) = 5)
CLS (1) <> 5
CLS -(1) = 5
LOCATE 1 = 1
LOCATE (1) * 2 = 2
WIDTH = 80
WIDTH (80) = 80
PRINT WIDTH(1)
