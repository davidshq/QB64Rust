$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): the error functions, one form: the four names without the underscore under $NOPREFIX
$NOPREFIX
PRINT ERRORLINE; INCLERRORLINE; "["; INCLERRORFILE$; "] ["; ERRORMESSAGE$(5); "] ["; ERRORMESSAGE$; "]"
PRINT "accepted"
SYSTEM
