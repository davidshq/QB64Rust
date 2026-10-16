$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.1): $DEBUG in a SUB never called, $DEBUG twice, and in an included file.
'$INCLUDE:'v33_inc/debug.bi'
$DEBUG
$IF _DEBUG_ THEN
    PRINT "debug on"
$END IF
SYSTEM
SUB s
    $DEBUG
END SUB
