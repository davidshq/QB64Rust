$Console:Only
' Slice (m4-builtins-param-types tasks 2.2, 3.3): _OS$, _MD5$, _CLIPBOARD$, _FILES$ and _CAST (verification/v49_a-d):
' the system's name by its parts, digests of strings of each kind, the clipboard set and read (up to a NUL),
' a folder listed by name, pattern and bare calls, and _CAST to each numeric type, truncated toward zero.
On Error GoTo handler
Dim s As String, f As String * 4, e As String, n As Long
Dim sg As Single, d As Double, k As Long, u As _Unsigned Long, q As _Integer64, b As _Bit
Print InStr(_OS$, "[WINDOWS]") > 0; Len(_OS$) > 0
Print _Md5$("abc"); " "; _Md5$("")
f = "ab": s = "QB64": Print _Md5$(f); " "; _Md5$(s + s); Len(_Md5$(String$(5000, 0)))
_Clipboard$ = "slice": Print "["; _Clipboard$; "]"
_Clipboard$ = "a" + Chr$(0) + "b": Print Len(_Clipboard$)
_Clipboard$ = f: Print Len(_Clipboard$)
_Clipboard$ = "": Print "["; _Clipboard$; "]"
e = _Files$
MkDir "s150_dir"
ChDir "s150_dir"
For n = 1 To 3
    Open "f" + LTrim$(Str$(n)) + ".txt" For Output As #1: Close #1
Next
Open "g.dat" For Output As #1: Close #1
e = _Files$("")
Do While Len(e): Print e; " ";: e = _Files$: Loop
Print
e = _Files$("*.txt"): n = 0
Do While Len(e): n = n + 1: e = _Files$: Loop
Print n; "["; _Files$("none*"); "]"; "["; _Files$("g.dat"); "]"
Kill "*.txt": Kill "g.dat"
ChDir ".."
RmDir "s150_dir"
sg = -6.75: d = 7.5: k = 300: u = 4294967295: q = 9007199254740993: b = -1
Print _Cast(_Byte, sg); _Cast(_Byte, k); _Cast(_Unsigned _Byte, u); _Cast(Integer, k * 300); _Cast(_Unsigned Integer, sg)
Print _Cast(Long, q); _Cast(_Unsigned Long, b); _Cast(_Integer64, d); _Cast(_Unsigned _Integer64, sg)
Print _Cast(_Offset, sg); _Cast(_Unsigned _Offset, k); _Cast(Single, q); _Cast(Double, q); _Cast(_Float, d / 3)
Print _Cast(Integer, 2.5); _Cast(Integer, -2.5); _Cast(Long, 0.9999); Hex$(_Cast(_Byte, -1)); " "; Hex$(_Cast(Integer, -1))
Print _Cast(_Byte, 100) + _Cast(_Byte, 100); _Cast(Integer, 30000) * 2; _Cast(_Byte, d * 100); _Cast(Long, d * 1E9)
k = _Cast(_Unsigned _Byte, 511): Print k; Str$(_Cast(Single, 2 / 3))
Print "done"
System
handler:
Print "handler"; Err
Resume Next
