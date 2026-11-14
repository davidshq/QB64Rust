$CONSOLE:ONLY
r = _SNDOPENRAW
PRINT "A handle:"; r > 0; "nothing queued:"; _SNDRAWLEN(r)
_SNDCLOSE r
END
