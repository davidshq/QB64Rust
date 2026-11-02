$CONSOLE:ONLY
' Verification (TODO.md, the 40-character limit on names): a label of 41 characters.
GOTO abcdefghijabcdefghijabcdefghijabcdefghijk
PRINT "skipped"
abcdefghijabcdefghijabcdefghijabcdefghijk:
PRINT "label"
SYSTEM
