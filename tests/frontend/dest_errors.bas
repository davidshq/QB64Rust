' TEST: check-fail
$CONSOLE
$SCREENHIDE
' _DEST, _SOURCE, _CONSOLE and _ECHO rejected (m2-console-screenhide task 3.1; verification\v26_x18-x20,
' x63-x68, x71): the statements _DEST and _SOURCE with no argument, a string, two arguments; _CONSOLE as the
' target of an assignment; _ECHO with a number, no argument, two arguments; the functions with an argument or
' with empty parentheses; after CALL with no argument or two
DIM d AS LONG, s AS STRING
_DEST
_DEST "a"
_DEST 0, 1
_SOURCE
_SOURCE s
_SOURCE 0, 1
_CONSOLE = 1
_ECHO 5
_ECHO
_ECHO "a", "b"
d = _DEST(1)
d = _DEST()
d = _SOURCE(1)
d = _SOURCE()
d = _CONSOLE(1)
d = _CONSOLE()
CALL _DEST
CALL _DEST(0, 1)
CALL _ECHO
