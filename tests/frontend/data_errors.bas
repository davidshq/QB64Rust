' TEST: check-fail
$CONSOLE:ONLY
' DATA, READ and RESTORE rejected (verification\v22_x65-x86): a READ target that is no variable (a literal, an
' expression, a CONST), a whole TYPE variable; RESTORE to a label that does not exist, to a label that two bodies
' have, with a type suffix; text after a quoted DATA item (the parser's error). Left out and "not supported yet"
' (data_marked.bas): RESTORE to a line number, a READ target in parentheses, a whole array; a built-in function
' call as a target is marked as an undeclared array is (the old compiler: "Expected variable")
CONST c = 5
TYPE pt
    x AS LONG
END TYPE
DIM v AS pt, n AS LONG
twice:
DATA 1, 2
READ 5
READ n + 1
READ c
READ v
RESTORE nowhere
RESTORE twice
RESTORE n&
DATA "a" x, 2
SYSTEM

SUB p
    twice:
    RESTORE twice
END SUB

FUNCTION fr&
    READ fr&
END FUNCTION
