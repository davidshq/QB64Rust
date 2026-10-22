' TEST: check-fail
$CONSOLE:ONLY
' STOP rejected (m4-qb64pe-sources task 4.1; verification\v52_x13): a string after it
STOP "x"
a$ = "b"
STOP a$ + "c"
