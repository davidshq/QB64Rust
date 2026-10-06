$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): GOSUB nested three deep (recursive).
depth = 0
GOSUB deep
PRINT "back, depth"; depth
SYSTEM
deep:
depth = depth + 1
PRINT "deep"; depth
IF depth < 3 THEN GOSUB deep
PRINT "leaving"; depth
RETURN
