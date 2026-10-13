' TEST: ir
$CONSOLE
$SCREENHIDE
' _DEST, _SOURCE and _ECHO in the IR (m2-console-screenhide task 3.1, design D5): one built-in operation each;
' the handle a LONG value (a float rounded, a wider integer narrowed, verification\v26_x21, x22), the text a
' string; after CALL the same operation (v26_x69, x70); in a SUB and a FUNCTION; _ECHO above the line of $CONSOLE
' in a file is the same statement (v26_x62)
DIM d AS LONG, i AS INTEGER, q AS _INTEGER64, sg AS SINGLE, f AS DOUBLE, s AS STRING, fx AS STRING * 4
_DEST _CONSOLE
_DEST 0
_DEST d
_DEST i
_DEST q
_DEST sg
_DEST f
_DEST 1.6
_DEST d + 1
_DEST (_DEST)
_SOURCE _CONSOLE
_SOURCE d
_SOURCE f
_SOURCE (_SOURCE)
_ECHO "text"
_ECHO s
_ECHO fx
_ECHO s + STR$(_DEST)
CALL _DEST(d)
CALL _SOURCE(_CONSOLE)
CALL _ECHO(s)
d = _DEST: _DEST d
toConsole
SYSTEM

SUB toConsole
    _DEST _CONSOLE
    _ECHO "in the sub"
END SUB

FUNCTION src&
    _SOURCE _CONSOLE
    src& = _SOURCE
END FUNCTION
