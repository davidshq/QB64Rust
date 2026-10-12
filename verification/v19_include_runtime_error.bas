$CONSOLE:ONLY
' An untrapped runtime error in an included file (m2-parser-breadth task 8.1): what does the message say about
' the line? (`qb64pe.bas` emits `evnt(linenumber, inclinenumber, "file")` for included code.)
PRINT "before"
'$INCLUDE:'v19_inc/raise.bi'
PRINT "after the include"
SYSTEM
