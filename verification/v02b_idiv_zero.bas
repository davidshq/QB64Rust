$CONSOLE:ONLY
' Verification: integer \ by zero under ON ERROR (v02 stops at MOD).
ON ERROR GOTO handler
z% = 0: x% = 7 \ z%: PRINT x%
PRINT "reached end"
SYSTEM
handler:
PRINT "trapped error"; ERR
RESUME NEXT
