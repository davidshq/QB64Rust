$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x54_const_after_marked_const)
CONST a = 3
CONST b = a OR UNDEFINED2
PRINT "ok"
SYSTEM
