$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-window-output row on arguments left out): _OPENFILEDIALOG$'s second argument left out; one form, on the last line, as code that is not reached.
PRINT "ok"
SYSTEM
x$ = _OPENFILEDIALOG$("t", , "*.txt")
