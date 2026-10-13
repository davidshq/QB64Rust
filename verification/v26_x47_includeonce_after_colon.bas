$CONSOLE:ONLY
' Verification (step 11, task 1.1): $INCLUDEONCE after a statement and a colon, in a file included twice.
'$INCLUDE:'v26_inc/colon_once.bi'
'$INCLUDE:'v26_inc/colon_once.bi'
PRINT "ok"
SYSTEM
