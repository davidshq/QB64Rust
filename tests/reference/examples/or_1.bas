$CONSOLE:ONLY
bold = 1: italic = 2: underline = 4
style = bold OR underline
PRINT "The style is"; style
key$ = "Q"
IF key$ = "q" OR key$ = "Q" THEN PRINT "Quit."
END
