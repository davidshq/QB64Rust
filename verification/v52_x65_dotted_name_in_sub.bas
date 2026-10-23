$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.3): does the old compiler accept this? (x65_dotted_name_in_sub)
' v.m in a SUB that does not share the main module's TYPE variable v (QB64pe const_eval.bas, LogicalNot&: num.typ):
' a variable of its own, of the default type? (1.5 stored shows SINGLE or an integer type.)
TYPE t
    m AS LONG
END TYPE
DIM v AS t
v.m = 7
s
s
PRINT v.m
SYSTEM
SUB s
    PRINT v.m
    v.m = 1.5
    PRINT v.m
    IF (v.m AND 1) THEN PRINT "odd"
END SUB
