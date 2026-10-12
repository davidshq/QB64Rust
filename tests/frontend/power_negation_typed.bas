' TEST: typed
$CONSOLE:ONLY
' A minus right of the power operator (found in m2-defxxx-common-arrays task 3.3 by upstream operators_test/op_exp;
' verification\v24_j_power_unary, study\02 section 1.2 step D): it takes what follows up to the next operator that
' is not ^, a negation or NOT, so the NOT inside it reaches only x; without the minus, and without the ^, NOT keeps
' its own reach
DIM x AS LONG, r AS DOUBLE
r = 2 ^ -NOT x + 3
r = 2 ^ -NOT x ^ 3 * 5
r = 2 ^ NOT x + 3
r = -NOT x + 3
r = 2 ^ -NOT NOT x = 2
SYSTEM
