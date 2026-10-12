' TEST: check-fail
' Precompiler flags (m2-console-screenhide task 6.1, the scenario "Nothing sets a flag"): in a program with none
' of the features every flag is 0, so none of these branches is compiled; each holds an error that would show
' if it were. The one diagnostic is the mark of a program without a console.
$IF _CONSOLE_ THEN
    x = "console"
$END IF
$IF _ASSERTS_ THEN
    x = "asserts"
$END IF
$IF _DEBUG_ THEN
    x = "debug"
$END IF
$IF _SOCKETS_ THEN
    x = "sockets"
$END IF
$IF _EXPLICIT_ OR _EXPLICITARRAY_ THEN
    x = "explicit"
$END IF
$IF _CONSOLE_ = 0 AND _EXPLICIT_ = 0 THEN
$ELSE
    x = "not 0"
$END IF
x = 1
