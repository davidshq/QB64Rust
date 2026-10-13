' Slice (m2-console-screenhide, group 3): _DEST, _SOURCE, _CONSOLE and _ECHO in a program with a console and a
' hidden window. Only what is printed while the console is the destination reaches standard output; _ECHO always
' does. No PRINT with a comma (tests\corpus\README.md).
$CONSOLE
$SCREENHIDE
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, l AS LONG, q AS _INTEGER64, sg AS SINGLE, db AS DOUBLE
DIM fl AS _FLOAT
ON ERROR GOTO handler

' Switching the destination, and the handles.
_ECHO "echo 1"
PRINT "window 1"
d& = _DEST
s& = _SOURCE
_DEST _CONSOLE
PRINT "console 1"
PRINT "before:"; d&; s&
PRINT "now:"; _DEST; _SOURCE; _CONSOLE
_DEST d&
PRINT "window 2"
_ECHO "echo 2: " + STR$(_DEST)
_DEST _CONSOLE
PRINT "console 2"
_DEST 0
PRINT "window 3"
_DEST _CONSOLE
PRINT "console 3"
_SOURCE _CONSOLE
PRINT "source:"; _SOURCE
_SOURCE s&
PRINT "source:"; _SOURCE

' Set in a procedure, it stays in force.
_DEST 0
toConsole
PRINT "console after the SUB"
_DEST 0
x = f
PRINT "console after the FUNCTION"; x; _SOURCE
_SOURCE 0

' A handle that does not exist: error 258, the destination and the source stay.
_DEST -99
PRINT "after the bad _DEST"; _DEST
_SOURCE -99
PRINT "after the bad _SOURCE"; _SOURCE

' The handle from every numeric type: a float is rounded, a wider integer narrowed.
c& = _CONSOLE
b = c&: i = c&: l = c&: q = c&: sg = c&: db = c&: fl = c&
_DEST 0: _DEST b: PRINT "byte"
_DEST 0: _DEST i: PRINT "integer"
_DEST 0: _DEST l: PRINT "long"
_DEST 0: _DEST q: PRINT "integer64"
_DEST 0: _DEST sg: PRINT "single"
_DEST 0: _DEST db: PRINT "double"
_DEST 0: _DEST fl: PRINT "float"
sg = -9.6: db = -10.4: fl = -10.5
_DEST 0: _DEST sg: PRINT "single -9.6"
_DEST 0: _DEST db: PRINT "double -10.4"
_DEST 0: _DEST fl: PRINT "float -10.5"
_DEST 0: _DEST -9.5: PRINT "literal -9.5"
q = 4294967286
_DEST 0: _DEST q: PRINT "integer64 4294967286"
ub = 0
_DEST ub: PRINT "window: unsigned byte 0"
_DEST b + 0: PRINT "expression"

' After CALL, in parentheses, in expressions.
_DEST 0
CALL _DEST(_CONSOLE)
CALL _SOURCE(_CONSOLE)
CALL _ECHO("echo 3")
PRINT "after CALL"; _DEST; _SOURCE
_SOURCE (0)
_DEST (_CONSOLE)
PRINT _DEST + _SOURCE; _CONSOLE * 2; -_DEST; _CONSOLE / 4; HEX$(_CONSOLE)
IF _DEST = _CONSOLE THEN PRINT "the console is the destination"
' The source is the window, whose handle reads as -2: neither CASE is taken and nothing is printed.
SELECT CASE _SOURCE
    CASE _CONSOLE: PRINT "the console is the source"
    CASE 0: PRINT "the window is the source"
END SELECT
SYSTEM

handler:
PRINT "error"; ERR
RESUME NEXT

SUB toConsole
    _DEST _CONSOLE
    PRINT "console in the SUB"
    _ECHO "echo in the SUB"
END SUB

FUNCTION f
    _DEST _CONSOLE
    _SOURCE _CONSOLE
    f = 7
END FUNCTION
