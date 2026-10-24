$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x84_colorchooser_statement)
' _COLORCHOOSERDIALOG as a statement.
k = 0
IF k = 1 THEN
    _COLORCHOOSERDIALOG "t"
END IF
PRINT "ok"
SYSTEM
