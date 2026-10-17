' TEST: cpp
$CONSOLE:ONLY
' The event checks the error functions read (m3-errorline-chr-const; verification\v43_b_include, v43_d_continued):
' in the main file the line, a line joined to the next by a continuation numbered as the last line of the join; in
' an included file, and in a file that one includes, the main file's line of the outermost $INCLUDE, the line in
' the file and the file's name; a label and a block of an included file the same.
ON ERROR GOTO h
ERROR 5: x = 1 + _
2
n& = _ERRORLINE + _INCLERRORLINE
s$ = _INCLERRORFILE$ + _ERRORMESSAGE$ + _ERRORMESSAGE$(n&)
'$INCLUDE:'inc/errorline.bi'
SYSTEM
h:
RESUME NEXT
