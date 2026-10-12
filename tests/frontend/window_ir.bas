' TEST: ir
$CONSOLE
$SCREENHIDE
' The window statements in the IR (m2-console-screenhide task 4.1, design D6; each call measured from the old
' compiler's C++ and compared by tests\callsite): one built-in operation each, every number a LONG value (a float
' rounded, a wider integer narrowed), a text a string, the pairs in parentheses as two slots each, _MOUSEHIDE's
' word as a choice. The Settings line names the icon part of the runtime, which _ICON needs (design D3). After
' CALL the statements without a template are the same operation (verification\v26_x74)
DIM l AS LONG, i AS INTEGER, q AS _INTEGER64, sg AS SINGLE, f AS DOUBLE, s AS STRING, fx AS STRING * 4
_DEST _CONSOLE
_SCREENHIDE
_SCREENSHOW
_SCREENICON
_TITLE "title"
_TITLE s
_TITLE fx
_TITLE s + "!"
_ICON
_ICON l
_ICON l, l
_ICON i, sg
_ICON 1.6, q
_MOUSESHOW
_MOUSESHOW "LINK"
_MOUSESHOW s
_MOUSEHIDE
_MOUSEHIDE _HIDE
_MOUSEHIDE _DISABLE
_MOUSECURSOR l
_MOUSECURSOR l, (l, l)
_MOUSECURSOR f, (i, q)
_MOUSECURSOR l + 1, (16, 16)
_WINDOWSIZELIMIT
_WINDOWSIZELIMIT (l, l)
_WINDOWSIZELIMIT -(l, l)
_WINDOWSIZELIMIT (l, l)-(l, l)
_WINDOWSIZELIMIT (320, 240)-(sg, f)
_FREEIMAGE
_FREEIMAGE l
_FREEIMAGE sg
_FREEIMAGE _NEWIMAGE(8, 8, 32)
CALL _SCREENHIDE
CALL _SCREENSHOW
CALL _SCREENICON
CALL _TITLE(s)
inSub
SYSTEM

SUB inSub
    _TITLE "in the sub"
    _SCREENICON
    _MOUSEHIDE
END SUB
