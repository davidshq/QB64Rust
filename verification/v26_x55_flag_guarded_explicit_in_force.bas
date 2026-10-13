$CONSOLE:ONLY
' Verification (step 11, task 1.4): as v26_x54, with a variable that is not declared. Is the option in force
' though its line is not compiled in the last round?
$IF _EXPLICIT_ = 0 THEN
    OPTION _EXPLICIT
$END IF
x = 1
PRINT "x ="; x
SYSTEM
