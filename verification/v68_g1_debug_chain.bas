$CONSOLE:ONLY
' v68 "Feature incompatible with $DEBUG mode": CHAIN in a program with $DEBUG, built outside the IDE.
$DEBUG
PRINT "start"
SYSTEM
CHAIN "v68_nothing.bas"
