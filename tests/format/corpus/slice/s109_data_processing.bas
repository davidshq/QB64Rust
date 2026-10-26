$Console:Only
' Slice (m3-asserts-debug-zlib task 3.1): the checksum, compression and base64 functions (verification\v33_b01,
' b02): checksums of strings of each kind, every compression level, a size given to _INFLATE$ (fewer bytes cut;
' more, and the bytes past the data are not set, so only their count is printed), bytes that are no zlib stream
' (zero bytes, no error), base64 of each length and of bad input.
On Error GoTo handler
Dim s As String, c As String, k As Long, d As Double, i64 As _Integer64, f As String * 4, u As _Unsigned Long
Print _Crc32("abc"); _Adler32("abc")
Print _Crc32(""); _Adler32("")
Print Hex$(_Crc32("The quick brown fox jumps over the lazy dog")); " "; Hex$(_Adler32("Wikipedia"))
f = "abc"
Print _Crc32(f); _Adler32(f)
k = _Crc32("abc"): Print k
u = _Adler32(s + "x"): Print u
i64 = _Crc32("abc") * 2: Print i64
Print Len(_Deflate$("this is a test", 9)); Len(_Deflate$("this is a test"))
s = String$(1000, "a")
For k = -2 To 12
    c = _Deflate$(s, k)
    Print k; Len(c); _Inflate$(c) = s
Next
c = _Deflate$(s)
Print Len(c); Len(_Inflate$(c)); Len(_Inflate$(c, 1000)); Len(_Inflate$(c, 10)); Len(_Inflate$(c, 5000))
Print Len(_Inflate$(c, 0)); Len(_Inflate$(c, -1)); Left$(_Inflate$(c, 1005), 1000) = s
Print Hex$(Asc(c, 1)); Hex$(Asc(c, 2)); Len(_Deflate$(""))
Print Len(_Inflate$(_Deflate$("")))
Print _Inflate$(_Deflate$("abc")) = "abc"; _Base64Decode$(_Base64Encode$("abc"))
d = 3.7: Print Len(_Deflate$(s, d)); Len(_Inflate$(c, d))
c = _Deflate$("hello world")
Print "["; _Inflate$(c, 3); "]"; Len(_Inflate$(Left$(c, 4), 5)); Len(_Inflate$("not deflated", 3))
s = _Inflate$("not deflated")
Print Len(s); s = String$(Len(s), 0)
Print "["; _Base64Encode$(""); "]["; _Base64Encode$("a"); "]["; _Base64Encode$("ab"); "]["; _Base64Encode$("abc"); "]"
Print _Base64Encode$(Chr$(0) + Chr$(255) + Chr$(254) + Chr$(62) + Chr$(63)); " "; _Base64Encode$(f)
Print "["; _Base64Decode$(""); "]["; _Base64Decode$("YQ=="); "]["; _Base64Decode$("YQ"); "]["; _Base64Decode$("YWJj"); "]"
Print "["; _Base64Decode$("!!!!"); "]"; Len(_Base64Decode$("Y Q = ="))
Print Len(_Base64Decode$("YWJ")); Len(_Base64Decode$("YWJjZ")); Len(_Base64Decode$("YW=j"))
Print "done"
System
handler:
Print "handler"; Err
Resume Next
Sub never
    Print _Inflate$("x")
End Sub
