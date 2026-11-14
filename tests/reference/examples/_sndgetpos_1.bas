$CONSOLE:ONLY
h = _SNDOPEN(Wav$(4000, 8000), "memory")
PRINT "At the start:"; _SNDGETPOS(h)
_SNDSETPOS h, .25
PRINT "After _SNDSETPOS h, .25:"; _SNDGETPOS(h)
_SNDSTOP h
PRINT "After _SNDSTOP:"; _SNDGETPOS(h)
END

' A WAV of silence: mono, 16 bits.
FUNCTION Wav$ (frames AS LONG, rate AS LONG)
    DIM d AS LONG
    d = frames * 2
    Wav$ = "RIFF" + MKL$(36 + d) + "WAVE" + "fmt " + MKL$(16) + MKI$(1) + MKI$(1) + MKL$(rate) + MKL$(rate * 2) + MKI$(2) + MKI$(16) + "data" + MKL$(d) + STRING$(d, 0)
END FUNCTION
