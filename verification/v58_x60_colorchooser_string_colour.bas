$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x60_colorchooser_string_colour)
' _COLORCHOOSERDIALOG with a string for the colour.
k = 0
IF k = 1 THEN
    x = _COLORCHOOSERDIALOG("t", "u")
END IF
PRINT "ok"
SYSTEM
