$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x15_bound_user_function)
TYPE t1
    v(f(1)) AS LONG
END TYPE
PRINT "ok"
SYSTEM
FUNCTION f (n)
    f = n
END FUNCTION
