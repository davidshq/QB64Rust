$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x53_const_undefined_suffix)
CONST c = 20 OR undefined%
PRINT c
SYSTEM
