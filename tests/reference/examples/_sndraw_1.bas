$CONSOLE:ONLY
r = _SNDOPENRAW
_SNDVOL r, 0
FOR i = 1 TO _SNDRATE: _SNDRAW 0, 0, r: NEXT
d# = _SNDRAWLEN(r)
PRINT "One second of samples queued, over 0 and at most 1:"; d# > 0; d# <= 1
END
