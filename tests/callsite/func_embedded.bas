$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
$EMBED:'./f.txt','h'
DIM s AS STRING
s = _EMBEDDED$("h")
END
