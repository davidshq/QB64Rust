$CONSOLE:ONLY
$CONSOLE
' Slice (m2-console-screenhide, group 6): the precompiler flags, second program. $CONSOLE:ONLY and below it
' $CONSOLE: _CONSOLE_ is 1, the value asked for last, and the program has no window. A setting inside a branch
' that a line above has switched off is met in no round: the OPTION _EXPLICIT inside $IF _CONSOLE_ = 0 stands
' below the lines that set the flag, so _EXPLICIT_ stays 0 and a variable needs no declaration. OPTION
' _EXPLICITARRAY on the last line sets _EXPLICITARRAY_ alone.
$IF _CONSOLE_ = 0 THEN
    OPTION _EXPLICIT
    PRINT "not printed"
$END IF
$IF _CONSOLE_ = 1 THEN
    PRINT "console 1"
$ELSEIF _CONSOLE_ = 2 THEN
    PRINT "console 2"
$ELSE
    PRINT "console 0"
$END IF
$IF _EXPLICIT_ THEN
    PRINT "explicit 1"
$ELSE
    PRINT "explicit 0"
$END IF
$IF _EXPLICITARRAY_ THEN
    PRINT "explicitarray 1"
$ELSE
    PRINT "explicitarray 0"
$END IF
notDeclared = 5
PRINT "a variable that is not declared:"; notDeclared
DIM a(3) AS LONG
a(1) = 7
PRINT "a declared array:"; a(1)
PRINT "destination:"; _DEST; _CONSOLE
SYSTEM
OPTION _EXPLICITARRAY
