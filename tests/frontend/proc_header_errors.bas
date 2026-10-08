' TEST: check-fail
$CONSOLE:ONLY
' A procedure whose header has an error gets that one error; its calls get none of their own (review of task 3.2):
' a reserved name, a reserved parameter name, a SUB with a suffix, and a missing END FUNCTION. (`cls 1` is the
' built-in CLS statement, read by its template since task 7.5 of m2-parser-breadth, so it is marked "not supported
' yet" itself.)
cls 1
CALL p("x")
v$ 2
PRINT f(1) + 1
SUB cls (a)
END SUB
SUB p (name AS STRING)
END SUB
SUB v$ (a)
END SUB
FUNCTION f (a)
