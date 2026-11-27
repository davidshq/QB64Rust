$CONSOLE:ONLY
' Verification (utf8-source, task 1.1): a file saved as UTF-8 that declares nothing. Its string holds
' the three bytes of the box-drawing character.
PRINT "─"
PRINT LEN("─"); ASC("─")
SYSTEM
