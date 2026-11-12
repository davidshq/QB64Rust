$CONSOLE:ONLY
PRINT "No such file:"; _SNDOPEN("no such file.wav")
h = _SNDOPEN(Wav$(4000, 8000), "memory")
PRINT "From memory:"; h; _SNDLEN(h)
END

' A WAV of silence: mono, 16 bits.
FUNCTION Wav$ (frames AS LONG, rate AS LONG)
    DIM d AS LONG
    d = frames * 2
    Wav$ = "RIFF" + MKL$(36 + d) + "WAVE" + "fmt " + MKL$(16) + MKI$(1) + MKI$(1) + MKL$(rate) + MKL$(rate * 2) + MKI$(2) + MKI$(16) + "data" + MKL$(d) + STRING$(d, 0)
END FUNCTION
