' TEST: check-fail
$CONSOLE:ONLY
' The forms of CHAIN the old compiler rejects (m3-console-files task 7.1; verification\v30_x95-x97), one line
' each: no name, a number, two arguments. Each is an error that is not "not supported yet"
DIM f AS STRING, n AS LONG
CHAIN
CHAIN 5
CHAIN "next", 5
CHAIN n
CALL CHAIN
CALL CHAIN(f, f)
n = CHAIN
