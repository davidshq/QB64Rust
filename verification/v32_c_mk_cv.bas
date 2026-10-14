$CONSOLE:ONLY
' Verification (m3-mem, task 1.1): MKI$, MKL$, MKS$, MKD$, CVI, CVL, CVS, CVD, _MK$ and _CV: the bytes of each
' value, printed as hex, and the value read back.
DIM s AS STRING, f AS STRING * 4, k AS LONG, d AS DOUBLE
PRINT LEN(MKL$(1)); CVL(MKL$(-2)); CVI(MKI$(300)); CVS(MKS$(1.5)); CVD(MKD$(2.25))
PRINT LEN(_MK$(_UNSIGNED _BYTE, 255)); _CV(_INTEGER64, _MK$(_INTEGER64, -9))
PRINT hx$(MKI$(1)); " "; hx$(MKI$(-1)); " "; hx$(MKI$(1.5)); " "; hx$(MKI$(2.5)); " "; hx$(MKI$(-2.5))
PRINT hx$(MKL$(&H12345678)); " "; hx$(MKL$(3000000000#)); " "; hx$(MKL$(-1))
PRINT hx$(MKS$(1)); " "; hx$(MKS$(-0.5)); " "; hx$(MKD$(1)); " "; hx$(MKD$(1 / 3))
PRINT hx$(_MK$(_BYTE, -1)); " "; hx$(_MK$(_UNSIGNED _BYTE, 256)); " "; hx$(_MK$(_UNSIGNED INTEGER, 65535))
PRINT hx$(_MK$(_UNSIGNED LONG, 4294967295#)); " "; hx$(_MK$(_INTEGER64, -2)); " "; hx$(_MK$(_UNSIGNED _INTEGER64, 5))
PRINT hx$(_MK$(_OFFSET, -1)); " "; hx$(_MK$(_UNSIGNED _OFFSET, 7)); " "; hx$(_MK$(_FLOAT, 1)); LEN(_MK$(_FLOAT, 1))
PRINT hx$(_MK$(_BIT, 1)); " "; hx$(_MK$(_BIT, 0)); " "; hx$(_MK$(_BIT, -1)); " "; hx$(_MK$(_UNSIGNED _BIT, 1)); " "; hx$(_MK$(_UNSIGNED _BIT, 3))
PRINT LEN(_MK$(_BIT, 1)); LEN(_MK$(_UNSIGNED _BIT, 1))
PRINT "read back"
s = CHR$(255) + CHR$(255) + CHR$(255) + CHR$(127) + CHR$(1) + CHR$(2) + CHR$(3) + CHR$(4) + STRING$(24, 0)
PRINT CVI(s); CVL(s); CVS(s); CVD(s)
PRINT _CV(_BYTE, s); _CV(_UNSIGNED _BYTE, s); _CV(INTEGER, s); _CV(_UNSIGNED INTEGER, s); _CV(LONG, s)
PRINT _CV(_UNSIGNED LONG, s); _CV(_INTEGER64, s); _CV(_UNSIGNED _INTEGER64, s); _CV(_OFFSET, s); _CV(_UNSIGNED _OFFSET, s)
PRINT _CV(SINGLE, s); _CV(DOUBLE, s); _CV(_FLOAT, MKD$(1) + STRING$(24, 0)); _CV(_FLOAT, _MK$(_FLOAT, 2.5))
PRINT _CV(_BIT, s); _CV(_UNSIGNED _BIT, s); _CV(_BIT, CHR$(0)); _CV(_UNSIGNED _BIT, CHR$(0))
PRINT "short strings"
ON ERROR GOTO handler
PRINT CVI(""): PRINT CVI("A"): PRINT CVL("AB"): PRINT CVD("ABC"): PRINT _CV(LONG, "A")
PRINT MKI$(0) = CHR$(0) + CHR$(0); _CV(_BIT, "")
PRINT CVI("ABC"); CVL("ABCDE")
ON ERROR GOTO 0
PRINT "a fixed-length string and an element"
f = MKL$(-7): PRINT CVL(f); _CV(LONG, f)
k = 5: d = 2.5: PRINT CVL(MKL$(k)); CVD(MKD$(d)); CVS(MKS$(d)); CVI(MKI$(d))
SYSTEM
handler:
PRINT "error"; ERR
RESUME NEXT

FUNCTION hx$ (s AS STRING)
    DIM i AS LONG, r AS STRING
    FOR i = 1 TO LEN(s)
        r = r + RIGHT$("0" + HEX$(ASC(s, i)), 2)
    NEXT
    hx$ = r
END FUNCTION
