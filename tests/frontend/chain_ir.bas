' TEST: ir
$CONSOLE:ONLY
' CHAIN in the IR (m3-console-files task 7.1, design D8; verification\v30_x98): a plain built-in statement with one
' string slot, written like a SUB call: a literal, a variable, a fixed-length string, an expression, one in
' parentheses, and after CALL
DIM f AS STRING, x AS STRING * 8, n AS LONG
IF n THEN CHAIN "next.bas"
IF n THEN CHAIN f
IF n THEN CHAIN x
IF n THEN CHAIN f + ".bas"
IF n THEN CHAIN ("next")
IF n THEN CALL CHAIN(f)
SYSTEM
