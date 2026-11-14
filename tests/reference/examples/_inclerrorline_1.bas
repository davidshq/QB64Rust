$CONSOLE:ONLY
ON ERROR GOTO handler
'$INCLUDE: 'risky.bi'
END

handler:
PRINT "Error"; ERR; "at main line"; _ERRORLINE
PRINT "In "; _INCLERRORFILE$; ", line"; _INCLERRORLINE
RESUME NEXT
