$CONSOLE:ONLY
' Verification (m4-upstream-rejections, follow-up): does the old compiler accept this? (x90_dim_string_prefix_type)
' A TYPE whose name starts with STRING, in DIM, REDIM, STATIC, SHARED and a parameter.
TYPE StringList
    n AS LONG
END TYPE
DIM a AS StringList
DIM SHARED g AS StringList
REDIM r(2) AS StringList
a.n = 1
g.n = 2
r(2).n = 3
s a
PRINT a.n; g.n; r(2).n
SYSTEM
SUB s (p AS StringList)
    STATIC t AS StringList
    t.n = t.n + p.n
    PRINT t.n
END SUB
