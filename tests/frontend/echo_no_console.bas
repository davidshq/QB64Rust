' TEST: check-fail
$SCREENHIDE
' _ECHO in a program with no console (m2-console-screenhide task 3.1, design D5; verification\v26_x17): each
' _ECHO is an error of the program, in the main module, after CALL and in a SUB; the program itself, one with a
' hidden window and no console, gets none (m3-window-output task 2.1), nor do _DEST and the three functions
_ECHO "x"
_DEST _CONSOLE
CALL _ECHO("y")
PRINT _DEST; _SOURCE
SYSTEM

SUB p
    _ECHO "z"
END SUB
