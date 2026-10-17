$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (65_expr_with_to_func)
DIM a(1 TO 5) AS LONG, b(0 TO 9) AS LONG
_ARRAYCOPY a(LBOUND(a) TO UBOUND(a)) TO b(LBOUND(b))
PRINT "ok"
SYSTEM
