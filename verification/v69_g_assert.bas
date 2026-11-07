$CONSOLE:ONLY
$ASSERTS
' Verification (OpenSpec change eval-order-left-to-right, task 2.1): the order in which _ASSERT evaluates its
' condition and its message (as verification\v33_a04_assert_evaluated_on, which Linux's g++ reverses). Every
' condition holds: a failed _ASSERT may open a dialog.
_ASSERT f(1), m$(2)
PRINT
_ASSERT f(3) > f(2), m$(4)
PRINT
_ASSERT f(5) _ANDALSO f(6), m$(7)
PRINT
PRINT "after"
SYSTEM

FUNCTION f& (k AS LONG)
    PRINT " f"; LTRIM$(STR$(k));
    f& = k
END FUNCTION

FUNCTION m$ (k AS LONG)
    PRINT " m"; LTRIM$(STR$(k));
    m$ = "message" + STR$(k)
END FUNCTION
