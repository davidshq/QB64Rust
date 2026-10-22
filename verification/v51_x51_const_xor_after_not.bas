$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x51_const_xor_after_not)
CONST c = 1 AND NOT XOR
PRINT "ok"
SYSTEM
