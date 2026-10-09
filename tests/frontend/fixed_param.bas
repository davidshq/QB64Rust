' TEST: typed
$CONSOLE:ONLY
' Parameters declared `STRING * n` or `t$n` (m2-numeric-types design D6, task 4.3; measured
' verification\v21_f_fixed_param): ordinary STRING parameters, passed by reference and changed uncut, except that
' `LEN` of the parameter named alone is the constant n. `t$4` is a name of its own (`t$` would be another variable).
' (`_UNSIGNED STRING`, with a length or not, is an error on a parameter: `numeric_decl_errors.bas`.)
s$ = "hello world"
p s$
PRINT "after p: ["; s$; "]"
r s$
PRINT "after r: ["; s$; "]"
PRINT g$("abcdefgh")
SYSTEM

SUB p (t AS STRING * 5)
    PRINT "["; t; "]"; LEN(t); LEN(t + "!"); LEN(RTRIM$(t))
    t = "much longer text"
    PRINT LEN(t)
END SUB

SUB r (t$4)
    PRINT "["; t$4; "]"; LEN(t$4)
    t$4 = "abcdefgh"
END SUB

FUNCTION g$ (t AS STRING * 3)
    g$ = t + "|" + STR$(LEN(t))
END FUNCTION
