$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.3): does the old compiler accept this? (x66_dotted_name_in_function_deflng)
' num.typ in a FUNCTION under DEFLNG that does not share the main module's TYPE variable num, read in an
' expression with a member of a parameter (QB64pe const_eval.bas, LogicalNot&, as string.bas includes it).
DEFLNG A-Z
TYPE pn
    typ AS LONG
END TYPE
TYPE ps
    num AS pn
END TYPE
DIM num AS pn, st AS ps
num.typ = 3
st.num.typ = 2
PRINT f&(st)
PRINT num.typ
SYSTEM
FUNCTION f& (state AS ps)
    IF (num.typ AND 2) OR (state.num.typ AND 2) THEN PRINT "set"
    num.typ = 1.5
    f& = num.typ + state.num.typ
END FUNCTION
