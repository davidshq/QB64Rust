' A tune: a scale played with SOUND, then a short tune played with PLAY.
_TITLE "A tune"
PRINT "A scale, note by note."
' The frequencies in hertz of a C major scale.
DATA 262,294,330,349,392,440,494,523
FOR i = 1 TO 8
    READ hertz
    PRINT hertz; "Hz"
    SOUND hertz, 4
NEXT
PRINT
PRINT "A tune: tempo 160, the fourth octave, quarter notes."
PLAY "T160 O4 L4 C C G G A A L2 G L4 F F E E D D L2 C"
PRINT "Done. Press any key."
SLEEP
SYSTEM
