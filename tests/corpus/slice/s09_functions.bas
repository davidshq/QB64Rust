$CONSOLE:ONLY
' FUNCTION results (spec language/procedures, "Calls"): each result type, printed with the function's own type;
' a call before the definition; the result assigned with and without the suffix; EXIT FUNCTION before any
' assignment (0 or ""); zero-argument functions; a function changing its argument inside a PRINT (by reference
' for n&, a copy for the SINGLE n).
PRINT "before definition:"; twice&(3)
PRINT twice&(3) + 1; twice&(twice&(2))
PRINT fi%(7); fl&(7); fq&&(7)
PRINT fs!(7); fd#(7); ff##(7)
PRINT nosuffix(7)
PRINT "["; fstr$("ab"); "]"
PRINT "rounded:"; round%(2.5); round%(3.5); round%(-2.5)
PRINT "assigned without suffix:"; noassign&(4)
PRINT "exit early:"; early&(1); "["; earlys$; "]"
PRINT "zero args:"; zero&; zero& + 1
n& = 1
PRINT incr&(n&); n&
n = 1
PRINT incr&(n); n
PRINT "x"; fstr$("y"); "z"
SYSTEM

FUNCTION twice& (a AS LONG)
    twice& = a * 2
END FUNCTION

FUNCTION fi% (a AS LONG)
    fi% = a * 1000
END FUNCTION

FUNCTION fl& (a AS LONG)
    fl& = a * 100000
END FUNCTION

FUNCTION fq&& (a AS LONG)
    fq&& = a * 10000000000&&
END FUNCTION

FUNCTION fs! (a AS LONG)
    fs! = a / 3
END FUNCTION

FUNCTION fd# (a AS LONG)
    fd# = a / 3
END FUNCTION

FUNCTION ff## (a AS LONG)
    ff## = a / 3
END FUNCTION

FUNCTION nosuffix (a AS LONG)
    nosuffix = a / 3
END FUNCTION

FUNCTION fstr$ (t AS STRING)
    fstr$ = t + t
END FUNCTION

FUNCTION round% (v AS DOUBLE)
    round% = v
END FUNCTION

FUNCTION noassign& (a AS LONG)
    noassign = a + 1
END FUNCTION

FUNCTION early& (a AS LONG)
    EXIT FUNCTION
    early& = 5
END FUNCTION

FUNCTION earlys$
    EXIT FUNCTION
    earlys$ = "x"
END FUNCTION

FUNCTION zero&
    zero& = 42
END FUNCTION

FUNCTION incr& (v AS LONG)
    v = v + 1
    incr& = v * 10
END FUNCTION
