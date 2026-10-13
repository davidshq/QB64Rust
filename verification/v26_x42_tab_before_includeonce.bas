$CONSOLE:ONLY
' Verification (step 11, task 1.1): a tab before $INCLUDEONCE, in a file included twice (compare v16_m7_twice:
' without the tab the file is taken once).
'$INCLUDE:'v26_inc/tab_once.bi'
'$INCLUDE:'v26_inc/tab_once.bi'
PRINT "ok"
SYSTEM
