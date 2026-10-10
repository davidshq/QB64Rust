$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "The rest"): RANDOMIZE without a seed under $CONSOLE:ONLY, with standard
' input at its end (run.sh gives the null device): does it prompt, wait, or go on?
PRINT "before"
RANDOMIZE
PRINT "after"
SYSTEM
