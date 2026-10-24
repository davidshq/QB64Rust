$CONSOLE:ONLY
' Verification (m4-builtins-remaining, review of step 4a): $ERRORLOCATION:ON and a DYNAMIC library that does not
' exist: is it error 259 as the program starts, naming the DECLARE line, or is it rejected when compiled?
$ERRORLOCATION:ON
PRINT "start"
DECLARE DYNAMIC LIBRARY "v58_no_such_library"
    FUNCTION NoSuchFunction&
END DECLARE
PRINT "not reached"
SYSTEM
