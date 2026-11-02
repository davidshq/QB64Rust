$CONSOLE:ONLY
' Verification (TODO.md, the 40-character limit on names): a word of 41 characters in a comment, a string and DATA.
' abcdefghijabcdefghijabcdefghijabcdefghijk
REM abcdefghijabcdefghijabcdefghijabcdefghijk
PRINT "abcdefghijabcdefghijabcdefghijabcdefghijk"
READ a$
PRINT a$
DATA abcdefghijabcdefghijabcdefghijabcdefghijk
SYSTEM
