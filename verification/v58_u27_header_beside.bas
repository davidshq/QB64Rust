$CONSOLE:ONLY
' Verification: a DECLARE LIBRARY in a $USELIBRARY top file whose header stands beside that file (test/header),
' found by the old compiler's fallback to the including file's folder.
$USELIBRARY:'test/header'
PRINT "twice:"; hd_twice&(21)
SYSTEM
