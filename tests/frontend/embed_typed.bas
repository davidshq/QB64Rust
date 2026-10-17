' TEST: typed
' $EMBED and _EMBEDDED$ (m3-network-embed design D1, D2): _EMBEDDED$ is a string, its argument the handle's literal;
' the lines in any letter case, with blanks and words around the quotes, without a colon or a comma, a handle that
' differs only in letter case, a line in a SUB above a call in a later SUB.
$CONSOLE:ONLY
$EMBED:'./a.txt','h'
$embed : ' ./b.txt ' , ' H ' words
$EMBED './c.txt''q'
DIM s AS STRING
s = _EMBEDDED$("h")
s = _EMBEDDED$("H") + _EMBEDDED$( "q" )
PRINT LEN(_EMBEDDED$("h"))
SYSTEM
SUB one
$EMBED:'./d.txt','inSub'
END SUB
SUB two
PRINT _EMBEDDED$("inSub")
END SUB
