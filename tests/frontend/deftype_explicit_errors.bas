' TEST: check-fail
$CONSOLE:ONLY
' Default types under OPTION _EXPLICIT (m2-defxxx-common-arrays task 2.1; verification\v24_x32): the message names
' the default type of the undeclared name at its line
OPTION _EXPLICIT
v = 5
DEFLNG A-Z
v = 5
DIM w
w = 1
DEFSTR W
w = "not the variable declared above"
SYSTEM
