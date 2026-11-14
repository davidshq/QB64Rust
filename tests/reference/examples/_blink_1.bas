$CONSOLE:ONLY
img& = _NEWIMAGE(80, 25, 0)
_DEST img&
_BLINK OFF
whenOff = _BLINK
_BLINK ON
whenOn = _BLINK
_DEST _CONSOLE
PRINT "_BLINK after OFF:"; whenOff; "and after ON:"; whenOn
END
