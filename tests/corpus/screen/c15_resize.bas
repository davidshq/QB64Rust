$RESIZE:ON
' Screen program (m4-qb64pe-builtins, task 2.1): $RESIZE:ON with no resize by the user: _RESIZE, _RESIZEWIDTH and
' _RESIZEHEIGHT at the start and after half a second, then after the statement _RESIZE OFF.
PRINT _RESIZE; _RESIZEWIDTH; _RESIZEHEIGHT
_DELAY .5
PRINT _RESIZE; _RESIZEWIDTH; _RESIZEHEIGHT
_RESIZE OFF
PRINT _RESIZE; _RESIZEWIDTH; _RESIZEHEIGHT
SYSTEM
