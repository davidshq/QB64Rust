' TEST: check-fail
$CONSOLE:ONLY
' ON STRIG rejected (m4-devices-continue-paths task 5.2, design D4; verification\v47_x04-x07, x09-x12, x21): no
' operands, three, empty parentheses, no first operand, no handler, a string operand, a SUB that needs an argument
' given none, an argument for a SUB that has none, a SUB whose parameter is SINGLE. Each is an error that is not
' "not supported yet"
ON STRIG GOSUB h
ON STRIG(1, 2, 3) GOSUB h
ON STRIG() GOSUB h
ON STRIG(, 1) GOSUB h
ON STRIG(1)
ON STRIG("a") GOSUB h
ON STRIG(0) s1
ON STRIG(0) s0 3
ON STRIG(0) sf 3
SYSTEM
h:
RETURN
SUB s0
END SUB
SUB s1 (n AS INTEGER)
END SUB
SUB sf (n AS SINGLE)
END SUB
