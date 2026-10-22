$CONSOLE
' Slice (m4-qb64pe-builtins, task 4.4): _CONSOLE OFF and ON in a $CONSOLE program (verification\v53_a_window),
' _EXIT with no request to end, and the drop built-ins with nothing dropped. _SCREENMOVE and the dialogs (task 4.3)
' as code that is not reached: they are never run.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _SCREENMOVE _MIDDLE
    _MESSAGEBOX "t", "m", "info"
    PRINT _MESSAGEBOX("t", "m", "yesno", "question", 0)
    PRINT _OPENFILEDIALOG$("t"); _SAVEFILEDIALOG$("t"); _SELECTFOLDERDIALOG$("t")
END IF
PRINT "a"
_CONSOLE OFF
PRINT "b"
_CONSOLE ON
PRINT "c"
PRINT _EXIT
_ACCEPTFILEDROP
PRINT _ACCEPTFILEDROP; _TOTALDROPPEDFILES
PRINT "["; _DROPPEDFILE$; "]["; _DROPPEDFILE(1); "]"
_FINISHDROP
_ACCEPTFILEDROP OFF
PRINT _ACCEPTFILEDROP
SYSTEM
