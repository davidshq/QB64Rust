$CONSOLE:ONLY
' Verification (m4-builtins-remaining, review of step 4a): $ERRORLOCATION:ON and a DYNAMIC library that exists
' without the declared function (error 260, raised as the program starts): the message names the FUNCTION line.
$ERRORLOCATION:ON
PRINT "start"
DECLARE DYNAMIC LIBRARY "kernel32"
    FUNCTION GetTickCount&
    FUNCTION V58NoSuchFunction&
END DECLARE
PRINT "not reached"
SYSTEM
