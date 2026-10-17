$CONSOLE:ONLY
' $EMBED and _EMBEDDED$ (m3-network-embed; verification\v44_a_embed, v44_x16, x22, x23, x28, x30, x35, x37-x39): the
' files in s138_embed.files\data\, each its length, CRC-32 and first bytes (every byte value, an empty file, 300,000
' bytes); a name with .\ and with .., blanks and words around the quotes, the metacommand's letter case and a longer
' name, blanks around the colon, no colon and no comma; handles that differ in letter case, one of 40 characters;
' a line in an included file (its ./ name beside this file); a file used twice, in a SUB, a FUNCTION, a sum, an
' argument, changed after a call; a call in a $IF branch not taken; a file never called for.
$EMBED:'./data/text.txt','text'
$EMBED:'./data/allbytes.bin','all256'
$EMBED:'./data/empty.bin','Empty'
$EMBED:'./data/big.txt','big'
$EMBED:'./data/noise.bin','noise'
$EMBED:'./data/odd.bin','odd'
$Embed:'.\data\text.txt','Back1'
$EMBED:' ./data/../data/allbytes.bin ',' Blanks9 '  trailing words
$EMBED:'./data/../data/noise.bin','dotdot'
$embed:'./data/lower.txt','lower'
$EMBEDX:'./data/longer.txt','longer'
$EMBED : './data/blanks.txt' , ' c ' words
$EMBED './data/noquote.txt''q'
$EMBED:'./data/case1.txt','h'
$EMBED:'./data/case2.txt','H'
$EMBED:'./data/long.txt','a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1'
$EMBED:'./data/unused.txt','unused' ' never called for, so not built in
'$INCLUDE:'inc/embed.bi'

show "text", _EMBEDDED$("text")
show "all256", _EMBEDDED$("all256")
show "Empty", _EMBEDDED$("Empty")
show "big", _EMBEDDED$("big")
show "noise", _EMBEDDED$("noise")
show "odd", _EMBEDDED$("odd")
show "Back1", _EMBEDDED$("Back1")
show "Blanks9", _EMBEDDED$("Blanks9")
show "dotdot", _EMBEDDED$("dotdot")
show "lower", _EMBEDDED$("lower")
show "longer", _EMBEDDED$("longer")
show "c", _EMBEDDED$("c")
show "q", _EMBEDDED$("q")
show "h", _EMBEDDED$("h")
show "H", _EMBEDDED$("H")
show "long", _EMBEDDED$("a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1")
show "fromInc", _EMBEDDED$("fromInc")
a$ = _EMBEDDED$("text")
MID$(a$, 1, 5) = "HELLO"
PRINT a$;
PRINT _EMBEDDED$("text");
PRINT "in a function:"; twice
PRINT "length of a sum:"; LEN(_EMBEDDED$("odd") + _EMBEDDED$("odd"))
PRINT "as an argument:"; ASC(_EMBEDDED$("all256"), 256)
PRINT "blanks in the call:"; LEN(_EMBEDDED$( "text" ))
PRINT "lines:"; _EMBEDDED$("lower"); "|"; _EMBEDDED$("H"); "|"; _EMBEDDED$("h")
$IF 0 THEN
    PRINT _EMBEDDED$("nothing")
$END IF
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
