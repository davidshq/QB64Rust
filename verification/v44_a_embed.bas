$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): $EMBED and _EMBEDDED$, the accepted forms.
' Each file: its length, its CRC-32 and its first bytes; a file used twice, in a SUB, a FUNCTION, and changed.
$EMBED:'./v44_files/text.txt','text'
$EMBED:'./v44_files/allbytes.bin','all256'
$EMBED:'./v44_files/empty.bin','Empty'
$EMBED:'./v44_files/big.txt','big'
$EMBED:'./v44_files/noise.bin','noise'
$EMBED:'./v44_files/odd.bin','odd'
$Embed:'.\v44_files\text.txt','Back1'
$EMBED:' ./v44_files/../v44_files/allbytes.bin ',' Blanks9 '  trailing words
$EMBED:'./v44_files/../v44_files/noise.bin','dotdot'

show "text", _EMBEDDED$("text")
show "all256", _EMBEDDED$("all256")
show "Empty", _EMBEDDED$("Empty")
show "big", _EMBEDDED$("big")
show "noise", _EMBEDDED$("noise")
show "odd", _EMBEDDED$("odd")
show "Back1", _EMBEDDED$("Back1")
show "Blanks9", _EMBEDDED$("Blanks9")
show "dotdot", _EMBEDDED$("dotdot")
a$ = _EMBEDDED$("text")
MID$(a$, 1, 5) = "HELLO"
PRINT a$;
PRINT _EMBEDDED$("text");
PRINT "in a function:"; twice
PRINT "length of a sum:"; LEN(_EMBEDDED$("odd") + _EMBEDDED$("odd"))
PRINT "as an argument:"; ASC(_EMBEDDED$("all256"), 256)
b$ = _EMBEDDED$("text"): PRINT "blanks in the call:"; LEN(_EMBEDDED$( "text" ))
PRINT "done"
SYSTEM

SUB show (n$, d$)
    PRINT n$; LEN(d$); HEX$(_CRC32(d$));
    FOR i& = 1 TO 6
        IF i& <= LEN(d$) THEN PRINT ASC(d$, i&);
    NEXT
    PRINT
END SUB

FUNCTION twice&
    twice& = LEN(_EMBEDDED$("big")) * 2
END FUNCTION
