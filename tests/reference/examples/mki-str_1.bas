$CONSOLE:ONLY
record$ = MKI$(300) + MKL$(-2) + MKS$(1.5) + MKD$(2.25)
PRINT "The record holds"; LEN(record$); "bytes"
PRINT CVI(MID$(record$, 1, 2)); CVL(MID$(record$, 3, 4))
PRINT CVS(MID$(record$, 7, 4)); CVD(MID$(record$, 11, 8))
END
