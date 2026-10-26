$Console:Only
' $EMBED and _EMBEDDED$ (m3-network-embed; verification\v44_a_embed, v44_x16, x22, x23, x28, x30, x35, x37-x39): the
' files in s138_embed.files\data\, each its length, CRC-32 and first bytes (every byte value, an empty file, 300,000
' bytes); a name with .\ and with .., blanks and words around the quotes, the metacommand's letter case and a longer
' name, blanks around the colon, no colon and no comma; handles that differ in letter case, one of 40 characters;
' a line in an included file (its ./ name beside this file); a file used twice, in a SUB, a FUNCTION, a sum, an
' argument, changed after a call; a call in a $IF branch not taken; a file never called for.
$Embed:'./data/text.txt','text'
$Embed:'./data/allbytes.bin','all256'
$Embed:'./data/empty.bin','Empty'
$Embed:'./data/big.txt','big'
$Embed:'./data/noise.bin','noise'
$Embed:'./data/odd.bin','odd'
$Embed:'.\data\text.txt','Back1'
$Embed:'./data/../data/allbytes.bin','Blanks9'  trailing words
$Embed:'./data/../data/noise.bin','dotdot'
$Embed:'./data/lower.txt','lower'
$Embed:'./data/longer.txt','longer'
$Embed:'./data/blanks.txt','c' words
$Embed:'./data/noquote.txt','q'
$Embed:'./data/case1.txt','h'
$Embed:'./data/case2.txt','H'
$Embed:'./data/long.txt','a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1'
$Embed:'./data/unused.txt','unused' ' never called for, so not built in
'$Include:'inc/embed.bi'

show "text", _Embedded$("text")
show "all256", _Embedded$("all256")
show "Empty", _Embedded$("Empty")
show "big", _Embedded$("big")
show "noise", _Embedded$("noise")
show "odd", _Embedded$("odd")
show "Back1", _Embedded$("Back1")
show "Blanks9", _Embedded$("Blanks9")
show "dotdot", _Embedded$("dotdot")
show "lower", _Embedded$("lower")
show "longer", _Embedded$("longer")
show "c", _Embedded$("c")
show "q", _Embedded$("q")
show "h", _Embedded$("h")
show "H", _Embedded$("H")
show "long", _Embedded$("a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1")
show "fromInc", _Embedded$("fromInc")
a$ = _Embedded$("text")
Mid$(a$, 1, 5) = "HELLO"
Print a$;
Print _Embedded$("text");
Print "in a function:"; twice
Print "length of a sum:"; Len(_Embedded$("odd") + _Embedded$("odd"))
Print "as an argument:"; Asc(_Embedded$("all256"), 256)
Print "blanks in the call:"; Len(_Embedded$("text"))
Print "lines:"; _Embedded$("lower"); "|"; _Embedded$("H"); "|"; _Embedded$("h")
$If 0 Then
    PRINT _EMBEDDED$("nothing")
$End If
Print "done"
System

Sub show (n$, d$)
    Print n$; Len(d$); Hex$(_Crc32(d$));
    For i& = 1 To 6
        If i& <= Len(d$) Then Print Asc(d$, i&);
    Next
    Print
End Sub

Function twice&
    twice& = Len(_Embedded$("big")) * 2
End Function
