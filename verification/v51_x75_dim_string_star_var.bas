$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x75_dim_string_star_var)
n = 2
DIM s AS STRING * n
PRINT "ok"
SYSTEM
