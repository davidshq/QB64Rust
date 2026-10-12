' TEST: check-fail
$CONSOLE:ONLY
' STRING members, whole values and parameters of a user type (m2-defxxx-common-arrays tasks 8.1, 8.2): the old
' compiler's errors as real errors (measured, verification\v24_x206, x208, x209, x211-x215).
TYPE t1
    x AS LONG
END TYPE
TYPE t2
    x AS LONG
END TYPE
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
TYPE outer
    i AS ts
END TYPE
DIM q AS t1, r AS t2, a(2) AS t1, s AS ts, sa(2) AS ts, o AS outer, n AS LONG
' "UDT must have fixed size": LEN of a value of a type with a STRING member, also in a member.
PRINT LEN(s)
PRINT LEN(sa(1))
PRINT LEN(o)
PRINT LEN(o.i)
' "Expected = similar user defined type": another TYPE, also one with the same members; a number; a member that
' is no TYPE; a value in parentheses.
q = r
a(1) = r
a(1) = 5
q = n
q = q.x
q = (q)
' "User defined types in expressions are invalid" and "TYPE 't1' required for sub": the argument for a parameter
' of a user type is a place of that type.
move r
move 5
move n
move q.x
PRINT f(r)
' The old compiler: "UNEXPECTED INTERNAL COMPILER ERROR!".
move (q)
move (a(1))
' A value of a user type for a parameter that is none.
one q
one a(1)
' A FUNCTION cannot have a user type as its result ("Expected (").
FUNCTION make AS t1
END FUNCTION
SUB move (p AS t1)
END SUB
SUB one (n AS LONG)
END SUB
FUNCTION f (p AS t1)
END FUNCTION
