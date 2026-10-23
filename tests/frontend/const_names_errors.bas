' TEST: check-fail
$CONSOLE:ONLY
' CONST values with an operator where a value is expected, or a name that is no constant defined above
' (m4-upstream-rejections, task 5.2), as measured (upstream const/bad_single_arg, not_bad_identifier,
' undefined_argument, undefined_argument_first; verification v51_x40 to x53): "Unexpected element 'OR'",
' "Expected variable/value after 'NOT'", and "Unexpected element 'UNDEFINED'" for an undefined name, a constant
' defined below, the name of a SUB, a label, a FUNCTION or an array, with a suffix too, also in a SUB. The name
' of a CONST above that was not taken keeps the mark ("not supported yet").
DIM arr(3) AS LONG
CONST a1 = OR
CONST a2 = NOT OR
CONST a3 = AND 1
CONST a4 = MOD
CONST a5 = 1 AND NOT XOR
CONST b1 = 20 OR UNDEFINED
CONST b2 = UNDEFINED OR 2
CONST b3 = later + 1
CONST later = 2
CONST b4 = s + 1
CONST b5 = here + 1
here:
CONST b6 = f + 1
CONST b7 = arr + 1
CONST b8 = 20 OR undefined%
SUB s
    CONST c1 = 20 OR UNDEFINED
    CONST taken = 1 / zz
    CONST c2 = taken + 1
END SUB
FUNCTION f
    f = 2
END FUNCTION
