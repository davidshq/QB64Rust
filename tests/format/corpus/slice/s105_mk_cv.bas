$Console:Only
' Slice (m3-mem task 5.1): verification\v32_c_mk_cv as a slice program. MKI$, MKL$, MKS$, MKD$, CVI, CVL, CVS,
' CVD, _MK$ and _CV: the bytes of each value printed as hex, the value read back, strings too short (error 5)
' and too long, a fixed-length string.
Dim s As String, f As String * 4, k As Long, d As Double
Print Len(MKL$(1)); CVL(MKL$(-2)); CVI(MKI$(300)); CVS(MKS$(1.5)); CVD(MKD$(2.25))
Print Len(_MK$(_Unsigned _Byte, 255)); _CV(_Integer64, _MK$(_Integer64, -9))
Print hx$(MKI$(1)); " "; hx$(MKI$(-1)); " "; hx$(MKI$(1.5)); " "; hx$(MKI$(2.5)); " "; hx$(MKI$(-2.5))
Print hx$(MKL$(&H12345678)); " "; hx$(MKL$(3000000000#)); " "; hx$(MKL$(-1))
Print hx$(MKS$(1)); " "; hx$(MKS$(-0.5)); " "; hx$(MKD$(1)); " "; hx$(MKD$(1 / 3))
Print hx$(_MK$(_Byte, -1)); " "; hx$(_MK$(_Unsigned _Byte, 256)); " "; hx$(_MK$(_Unsigned Integer, 65535))
Print hx$(_MK$(_Unsigned Long, 4294967295#)); " "; hx$(_MK$(_Integer64, -2)); " "; hx$(_MK$(_Unsigned _Integer64, 5))
Print hx$(_MK$(_Offset, -1)); " "; hx$(_MK$(_Unsigned _Offset, 7)); " "; hx$(_MK$(_Float, 1)); Len(_MK$(_Float, 1))
Print hx$(_MK$(_Bit, 1)); " "; hx$(_MK$(_Bit, 0)); " "; hx$(_MK$(_Bit, -1)); " "; hx$(_MK$(_Unsigned _Bit, 1)); " "; hx$(_MK$(_Unsigned _Bit, 3))
Print Len(_MK$(_Bit, 1)); Len(_MK$(_Unsigned _Bit, 1))
Print "read back"
s = Chr$(255) + Chr$(255) + Chr$(255) + Chr$(127) + Chr$(1) + Chr$(2) + Chr$(3) + Chr$(4) + String$(24, 0)
Print CVI(s); CVL(s); CVS(s); CVD(s)
Print _CV(_Byte, s); _CV(_Unsigned _Byte, s); _CV(Integer, s); _CV(_Unsigned Integer, s); _CV(Long, s)
Print _CV(_Unsigned Long, s); _CV(_Integer64, s); _CV(_Unsigned _Integer64, s); _CV(_Offset, s); _CV(_Unsigned _Offset, s)
Print _CV(Single, s); _CV(Double, s); _CV(_Float, MKD$(1) + String$(24, 0)); _CV(_Float, _MK$(_Float, 2.5))
Print _CV(_Bit, s); _CV(_Unsigned _Bit, s); _CV(_Bit, Chr$(0)); _CV(_Unsigned _Bit, Chr$(0))
Print "short strings"
On Error GoTo handler
Print CVI(""): Print CVI("A"): Print CVL("AB"): Print CVD("ABC"): Print _CV(Long, "A")
Print MKI$(0) = Chr$(0) + Chr$(0); _CV(_Bit, "")
Print CVI("ABC"); CVL("ABCDE")
On Error GoTo 0
Print "a fixed-length string and an element"
f = MKL$(-7): Print CVL(f); _CV(Long, f)
k = 5: d = 2.5: Print CVL(MKL$(k)); CVD(MKD$(d)); CVS(MKS$(d)); CVI(MKI$(d))
System
handler:
Print "error"; Err
Resume Next

Function hx$ (s As String)
    Dim i As Long, r As String
    For i = 1 To Len(s)
        r = r + Right$("0" + Hex$(Asc(s, i)), 2)
    Next
    hx$ = r
End Function
