' TEST: check-fail
$CONSOLE:ONLY
' Precompiler flags (m2-console-screenhide task 6.2, design D7; verification/v26_x55): the only OPTION _EXPLICIT
' stands inside the branch its own flag switches off, so its line is not compiled in the last round, and the
' option is in force all the same: a variable and an array that are not declared are errors, as in the old
' compiler ("Variable 'x' (SINGLE) not defined").
$IF _EXPLICIT_ = 0 THEN
    OPTION _EXPLICIT
$END IF
x = 1
a(2) = 3
SYSTEM
