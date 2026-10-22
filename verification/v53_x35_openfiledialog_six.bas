$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x35_openfiledialog_six)
' _OPENFILEDIALOG$ with six arguments.
k = 0
IF k = 1 THEN
    f$ = _OPENFILEDIALOG$("t", "", "*.bas", "BASIC", 0, 1)
END IF
PRINT "ok"
SYSTEM
