$CONSOLE:ONLY
' _ENCODEURL$ and _DECODEURL$, the functions of QB64pe's file included after the last line (m3-network-embed;
' verification\v44_d_url, v44_c_flags, v44_x54, x55): each string encoded and decoded again, a bad and a cut
' percent sequence, a plus, lower-case digits, the functions in an expression, without their $, in a SUB and a
' FUNCTION. _OPENCLIENT in a SUB never called makes _SOCKETS_ 1, so the file's helper for libqb is compiled too.
$IF _SOCKETS_ THEN
    PRINT "sockets on"
$ELSE
    PRINT "sockets off"
$END IF
show "http://w%w.example&.com/"
show "a b?x=1&y=2 3"
show "?a&b=c=d&&e"
show "path/with spaces/and#hash?q=[1]"
show "user@host:8080/x?k=a@b:c"
show ""
show CHR$(0) + CHR$(127) + CHR$(128) + CHR$(255)
show "-._~09AZaz"
PRINT "decode bad: ["; _DECODEURL$("%41%42%zz%4"); "]"; LEN(_DECODEURL$("%zz"))
PRINT "decode end: ["; _DECODEURL$("abc%"); "]"; LEN(_DECODEURL$("abc%"))
PRINT "decode plus: ["; _DECODEURL$("a+b%2Bc"); "]"
PRINT "decode lower: ["; _DECODEURL$("%4a%4A"); "]"
PRINT "in an expression:"; LEN(_ENCODEURL$("a b") + _ENCODEURL$("c d"))
PRINT "without $: "; _ENCODEURL("a b"); " "; _DECODEURL("a%20b")
PRINT "from a function: "; twice$("x y")
SYSTEM

SUB show (u$)
    e$ = _ENCODEURL$(u$)
    PRINT "["; e$; "] ["; _DECODEURL$(e$); "]"; LEN(e$); _DECODEURL$(e$) = u$
END SUB

FUNCTION twice$ (s$)
    twice$ = _ENCODEURL$(s$) + "|" + _ENCODEURL$(_ENCODEURL$(s$))
END FUNCTION

SUB never
    PRINT _OPENCLIENT("TCP/IP:1:localhost")
END SUB
