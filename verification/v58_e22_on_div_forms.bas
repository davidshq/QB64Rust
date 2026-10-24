$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:ON and the critical error in a single-line IF, after a joined line,
' in a SUB (called from line 9), chosen by a value read from DATA: one program per run, so each is its own program
' here (v58_e22, the single-line IF).
$ERRORLOCATION:ON
PRINT "start"
z% = 0
IF z% = 0 THEN PRINT "in if": q% = 7 \ z%
PRINT "not reached"
SYSTEM
