' TEST: check-fail
$CONSOLE
$SCREENHIDE
' The window built-ins rejected (m2-console-screenhide task 4.1; each form one program of
' verification\v26_x75-x113, where the old compiler rejects it too): a statement with too many arguments, with an
' argument of the wrong kind, with a pair that is not a pair, with a word that is not one of its choices, without
' its argument; a function with empty parentheses, with an argument it does not take, with too few or too many,
' with a string for a number, without its `$`, as the target of an assignment, as a statement. ERR and ERL with
' an argument are the same error (x76, x77). Left "not supported yet" (the old compiler accepts them, x74): the
' statements that have a template after CALL, the function _SCREENHIDE, and _SCREENEXISTS as the argument of
' HEX$, OCT$ or _BIN$, where the old compiler passes another width than for a LONG
DIM n AS LONG, s AS STRING
n = _SCREENEXISTS(1)
n = ERR(1)
n = ERL(1)
_ICON 1, 2, 3
_ICON "a"
_MOUSESHOW 1
_MOUSESHOW "a", "b"
_MOUSEHIDE 1
_MOUSEHIDE _SHOW
_MOUSECURSOR
_MOUSECURSOR 1, 2, 3
_MOUSECURSOR 1, (2)
_WINDOWSIZELIMIT 320, 240
_WINDOWSIZELIMIT (320)
_WINDOWSIZELIMIT (1, 2)-(3)
_FREEIMAGE "a"
_FREEIMAGE 1, 2
_TITLE
_TITLE 1
_TITLE "a", "b"
_SCREENHIDE 1
_SCREENSHOW 1
_SCREENICON 1
n = _SCREENX()
n = _SCREENX(1)
n = _SCREENEXISTS()
s = _TITLE$()
s = _TITLE$(1)
n = _TITLE
n = _NEWIMAGE(1)
n = _NEWIMAGE(1, 2, 3, 4)
n = _NEWIMAGE("a", 2)
n = _NEWIMAGE
n = _WINDOWHANDLE()
n = _WINDOWHANDLE(1, 2)
n = _WINDOWHANDLE("a")
_SCREENX = 1
_NEWIMAGE 1, 2
n = _DESKTOPWIDTH(1)
CALL _ICON
CALL _ICON(1, 2)
CALL _FREEIMAGE(1)
CALL _MOUSESHOW("a")
CALL _MOUSEHIDE(_HIDE)
CALL _MOUSECURSOR(1, (2, 3))
CALL _WINDOWSIZELIMIT((1, 2)-(3, 4))
n = _SCREENHIDE
s = HEX$(_SCREENEXISTS)
s = OCT$(_SCREENEXISTS)
s = _BIN$(_SCREENEXISTS)
