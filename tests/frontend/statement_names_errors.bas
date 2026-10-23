' TEST: check-fail
$CONSOLE:ONLY
' Statements that name no SUB (m4-names-implicit-arrays task 2.3): the old compiler's errors as real errors
' (measured, verification\v17_b_if_then_label, v20_x06, v22_x20, v33_x50, v55_x21 to x34). The lines between the
' errors are accepted.
DIM v AS LONG, c AS LONG
v = 1
' "Syntax error": a name that is no SUB, with or without arguments, a variable's name, after THEN.
x 1
UndefinedSub
v
IF c THEN nosub
IF c THEN nosub 1
IF c THEN jumped
' "Expected CALL sub-name [(...)]".
CALL nosub
CALL nosub(1)
' "Syntax error": a built-in statement or a keyword with a suffix it does not take.
KILL$ "a"
_ASSERT$ 1
CLS$
BEEP%
PRINT$ "a"
' "Syntax error": a built-in function as a statement.
LEN("a")
LEN "a"
ABS(1)
' Accepted: a SUB, also after THEN and CALL.
here 1
IF c THEN here 2
CALL here(3)
SYSTEM
jumped:

SUB here (n)
    PRINT n
END SUB
