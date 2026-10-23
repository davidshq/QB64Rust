$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x85_redim_typo_in_sub)
s
PRINT "ok"
SYSTEM
SUB s
    REDIM a(3) AS nosuch
END SUB
