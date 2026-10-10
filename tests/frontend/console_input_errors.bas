' TEST: check-fail
$CONSOLE:ONLY
' Console INPUT and LINE INPUT rejected (m2-builtin-statements task 6.4; verification\v22_x138-x158): a target that
' is no variable (a literal, an expression, a CONST), no target, a whole TYPE variable; a prompt that is no string
' literal (a variable or an expression before ; is taken as the first target, and the ; is the error); a prompt
' with nothing after it; LINE INPUT into a number or into two targets; the FUNCTION's own name. Left out and "not
' supported yet": a member of an element
CONST c = 5
TYPE pt
    x AS LONG
END TYPE
DIM l AS LONG, s AS STRING, p AS STRING, v AS pt
INPUT 5
INPUT l + 1
INPUT c
INPUT v
INPUT
INPUT "prompt"
INPUT "prompt";
INPUT p; l
INPUT p + "x"; l
INPUT "a" "b"; l
INPUT l, , s
INPUT l,,
INPUT #1, l,
LINE INPUT #1, s,
LINE INPUT l
LINE INPUT s, p
LINE INPUT
LINE INPUT "prompt";
SYSTEM

FUNCTION fr&
    INPUT fr&
END FUNCTION

FUNCTION fs$
    LINE INPUT fs$
END FUNCTION
