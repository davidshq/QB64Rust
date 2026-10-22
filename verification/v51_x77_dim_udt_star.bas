$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x77_dim_udt_star)
TYPE pt
    x AS LONG
END TYPE
DIM v AS pt * 2
PRINT "ok"
SYSTEM
