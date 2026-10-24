$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x34_consolecursor_number)
' _CONSOLECURSOR with a number for the word.
k = 0
IF k = 1 THEN
    _CONSOLECURSOR 1
END IF
PRINT "ok"
SYSTEM
