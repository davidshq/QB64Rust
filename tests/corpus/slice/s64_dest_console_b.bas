' Slice (m2-console-screenhide, group 3): _DEST, _SOURCE, _CONSOLE and _ECHO in a program without a window. The
' console is the destination and the source from the first $CONSOLE:ONLY; _DEST 0 takes the output away from
' standard output, and $CONSOLE:ONLY a second time selects the console again where it stands. _ECHO stands above
' the first $CONSOLE:ONLY too.
_ECHO "echo above"
$CONSOLE:ONLY
PRINT "console 1"; _DEST; _CONSOLE; _SOURCE
_ECHO "echo 1"
_DEST 0
PRINT "window"
_ECHO "echo 2:" + STR$(_DEST)
$CONSOLE:ONLY
PRINT "console 2"; _DEST
_DEST 0: _SOURCE 0
PRINT "window"; _SOURCE
_DEST _CONSOLE
PRINT "console 3"; _DEST; _SOURCE
SYSTEM
