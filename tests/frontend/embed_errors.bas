' TEST: check-fail
' The forms of $EMBED and _EMBEDDED$ the old compiler rejects that sema decides (m3-network-embed design D1, D2;
' verification/v44_x01-x06, x08-x15, x17, x18, x20, x21, x24, x25, x29, x31-x34, x36): no quotes, an empty file or
' handle, a bad handle, a file or a handle twice, a call that is not one literal, empty, of a bad handle, of no
' line, of a line below it, in a SUB, in a branch not taken, in a comment, without its argument or with two, in a
' CONST; last, one file as ./ and as .\ (x69; another separator after ./ is another file, x70, accepted). Each is an
' error, none "not supported yet". A missing file or a folder is the driver's check (CLI test); `_EMBEDDED` without
' its $ (x19) and a name relative to the compiler's folder (x26) are "not supported yet".
$CONSOLE:ONLY
PRINT LEN(_EMBEDDED$("early"))
$EMBED
$EMBED:'./f.txt'
$EMBED:'','h'
$EMBED:'./f.txt',''
$EMBED:'./f.txt','1abc'
$EMBED:'./f.txt','my_h'
$EMBED:'./a.txt','h'
$EMBED:'./a.txt','other'
$EMBED:'./b.txt','h'
$EMBED:'./c.txt','early'
$EMBED:'./d.txt','ABC'
'$EMBED:'./e.txt','incomment'
$IF 0 THEN
$EMBED:'./g.txt','skipped'
$END IF
h$ = "h"
PRINT LEN(_EMBEDDED$(h$))
PRINT LEN(_EMBEDDED$(""))
PRINT LEN(_EMBEDDED$("abc"))
PRINT LEN(_EMBEDDED$("nope"))
PRINT LEN(_EMBEDDED$("a" + "b"))
PRINT LEN(_EMBEDDED$)
PRINT LEN(_EMBEDDED$("h", 1))
PRINT LEN(_EMBEDDED$("incomment"))
PRINT LEN(_EMBEDDED$("skipped"))
PRINT LEN(_EMBEDDED$(" h "))
PRINT LEN(_EMBEDDED$(CHR$(104)))
PRINT LEN(_EMBEDDED$("insub"))
PRINT LEN(_EMBEDDED$("below"))
CONST k = _EMBEDDED$("h")
SYSTEM
SUB s
$EMBED:'./s.txt','insub'
END SUB
$EMBED:'./x.txt','below'
$EMBED:'.\x.txt','backslash'
$EMBED:'./y/z.txt','slash'
$EMBED:'./y\z.txt','backslash2'
