$CONSOLE:ONLY
r = _SNDOPENRAW
PRINT "Nothing queued:"; _SNDRAWLEN(r)
_SNDVOL r, 0
FOR i = 1 TO _SNDRATE \ 2: _SNDRAW 0, 0, r: NEXT
PRINT "Half a second queued, over 0:"; _SNDRAWLEN(r) > 0
END
