$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x73_dim_type_below)
DIM v AS pt
v.x = 4
PRINT v.x
SYSTEM
TYPE pt
    x AS LONG
END TYPE
