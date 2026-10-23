$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x89_array_param_typo)
PRINT "ok"
SYSTEM
SUB s (b() AS _Bytef)
END SUB
