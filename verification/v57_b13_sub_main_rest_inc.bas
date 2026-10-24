$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a SUB closed in an included file whose later lines are module level.
s
PRINT "main"
SYSTEM
SUB s
'$INCLUDE:'v57_inc/endsub_more.bi'
