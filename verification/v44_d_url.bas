$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _ENCODEURL$ and _DECODEURL$ (afterlastline.bm) in a
' program without network functions, so _SOCKETS_ is 0 and _WhatIsMyIP$ is not compiled.
$IF _SOCKETS_ THEN
    PRINT "sockets on"
$ELSE
    PRINT "sockets off"
$END IF
show "http://w%w.example&.com/"
show "a b?x=1&y=2 3"
show "?a&b=c=d&&e"
show "path/with spaces/and#hash?q=[1]"
show ""
show CHR$(0) + CHR$(127) + CHR$(128) + CHR$(255)
show "-._~09AZaz"
PRINT "decode bad:"; "["; _DECODEURL$("%41%42%zz%4"); "]"; LEN(_DECODEURL$("%zz"))
PRINT "decode end:"; "["; _DECODEURL$("abc%"); "]"; LEN(_DECODEURL$("abc%"))
PRINT "decode plus:"; "["; _DECODEURL$("a+b%2Bc"); "]"
PRINT "decode lower:"; "["; _DECODEURL$("%4a%4A"); "]"
PRINT "in an expression:"; LEN(_ENCODEURL$("a b") + _ENCODEURL$("c d"))
SYSTEM

SUB show (u$)
    e$ = _ENCODEURL$(u$)
    PRINT "["; e$; "] ["; _DECODEURL$(e$); "]"; LEN(e$)
END SUB
