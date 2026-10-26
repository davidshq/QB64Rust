$Console:Only
' _ENCODEURL$ and _DECODEURL$, the functions of QB64pe's file included after the last line (m3-network-embed;
' verification\v44_d_url, v44_c_flags, v44_x54, x55): each string encoded and decoded again, a bad and a cut
' percent sequence, a plus, lower-case digits, the functions in an expression, without their $, in a SUB and a
' FUNCTION. _OPENCLIENT in a SUB never called makes _SOCKETS_ 1, so the file's helper for libqb is compiled too.
$If _SOCKETS_ Then
    Print "sockets on"
$Else
    PRINT "sockets off"
$End If
show "http://w%w.example&.com/"
show "a b?x=1&y=2 3"
show "?a&b=c=d&&e"
show "path/with spaces/and#hash?q=[1]"
show "user@host:8080/x?k=a@b:c"
show ""
show Chr$(0) + Chr$(127) + Chr$(128) + Chr$(255)
show "-._~09AZaz"
Print "decode bad: ["; _DecodeURL$("%41%42%zz%4"); "]"; Len(_DecodeURL$("%zz"))
Print "decode end: ["; _DecodeURL$("abc%"); "]"; Len(_DecodeURL$("abc%"))
Print "decode plus: ["; _DecodeURL$("a+b%2Bc"); "]"
Print "decode lower: ["; _DecodeURL$("%4a%4A"); "]"
Print "in an expression:"; Len(_EncodeURL$("a b") + _EncodeURL$("c d"))
Print "without $: "; _EncodeURL("a b"); " "; _DecodeURL("a%20b")
Print "from a function: "; twice$("x y")
System

Sub show (u$)
    e$ = _EncodeURL$(u$)
    Print "["; e$; "] ["; _DecodeURL$(e$); "]"; Len(e$); _DecodeURL$(e$) = u$
End Sub

Function twice$ (s$)
    twice$ = _EncodeURL$(s$) + "|" + _EncodeURL$(_EncodeURL$(s$))
End Function

Sub never
    Print _OpenClient("TCP/IP:1:localhost")
End Sub
