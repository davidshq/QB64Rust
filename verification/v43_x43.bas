$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): the error functions, one form: the types of the results, through a variable of each
a& = _ERRORLINE: b& = _INCLERRORLINE: c$ = _INCLERRORFILE$: d$ = _ERRORMESSAGE$: PRINT a&; b&; c$; d$
PRINT _ERRORLINE / 2; _ERRORLINE + 2147483647; _INCLERRORLINE + 2147483647
PRINT "accepted"
SYSTEM
