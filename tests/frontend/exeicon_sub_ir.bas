' TEST: ir
$CONSOLE:ONLY
' $EXEICON inside a SUB (m3-sound task 4.1; verification\v29_e15, e10, e09, e20): the statement is the SUB's, the
' icon the program's. Lower case, blanks around the name, letters after the word and words after the name are the
' same line to the old compiler
PRINT "ok"
SYSTEM

SUB one
       $exeicons:'  ./inc/exeicon.ico  ' and more ' a comment
END SUB
