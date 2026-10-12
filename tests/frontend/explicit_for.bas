' TEST: check-fail
$CONSOLE:ONLY
' Under OPTION _EXPLICIT a FOR variable is not a declaration (m2-control-flow-slice D7, task 5.2,
' verification\v17_f_explicit_for); a DIMmed one is fine
OPTION _EXPLICIT
DIM k AS INTEGER
FOR k = 1 TO 2: NEXT
FOR i = 1 TO 2
NEXT
