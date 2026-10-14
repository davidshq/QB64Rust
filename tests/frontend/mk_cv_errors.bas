' TEST: check-fail
$CONSOLE:ONLY
' The old compiler's errors for MKI$ ... _CV (m3-mem task 5.1; verification\v32_x96 to x113), one per line: a width
' or length in the type, STRING, a string where a number goes and the other way round, the counts, a word or a
' variable that is no type.
DIM s AS STRING, k AS LONG
s = _MK$(_BIT * 4, k)
k = _CV(_BIT * 4, s)
k = _CV(_UNSIGNED _BIT * 4, s)
s = _MK$(STRING, k)
k = _CV(STRING, s)
s = _MK$(STRING * 4, k)
s = _MK$(LONG, "a")
k = CVL(5)
k = CVI(k)
k = _CV(LONG, 5)
s = MKL$("a")
s = MKI$
s = MKI$(1, 2)
s = _MK$(LONG)
s = _MK$(FOO, 1)
k = _CV(FOO, s)
s = _MK$(LONG, 1, 2)
s = _MK$(k, 1)
SYSTEM
