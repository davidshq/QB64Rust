$CONSOLE:ONLY
' Verification (m2-parser-breadth, M7): the same file included twice; $INCLUDEONCE first line and later line.
'$INCLUDE:'v16_inc_a.bi'
'$INCLUDE:'v16_inc_a.bi'
'$INCLUDE:'v16_inc/once.bi'
'$INCLUDE:'v16_inc/once.bi'
'$INCLUDE:'./v16_inc/once.bi'
'$INCLUDE:'v16_inc/once_late.bi'
'$INCLUDE:'v16_inc/once_late.bi'
SYSTEM
