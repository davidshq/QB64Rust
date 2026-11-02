$CONSOLE:ONLY
' Verification (TODO.md, the 40-character limit on names): a DECLARE LIBRARY name of 41 characters with an ALIAS.
DECLARE LIBRARY
    FUNCTION abcdefghijabcdefghijabcdefghijabcdefghijk& ALIAS "abs" (BYVAL n AS LONG)
END DECLARE
PRINT abcdefghijabcdefghijabcdefghijabcdefghijk&(-3)
SYSTEM
