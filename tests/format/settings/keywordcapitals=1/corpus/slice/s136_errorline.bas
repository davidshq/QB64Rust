$CONSOLE:ONLY
' _ERRORLINE, _INCLERRORLINE, _INCLERRORFILE$ and _ERRORMESSAGE$ in the main file (m3-errorline-chr-const;
' verification\v43_a_errorline, v43_d_continued, v43_x44): before any error, after errors on several lines, after
' RESUME NEXT, in a SUB, a FUNCTION and a GOSUB, a line with several statements, blank lines and comments, lines
' continued with an underscore (a line joined to the next is numbered as the last line of the join), a line in a
' $IF branch, the message of codes given as an argument, rounded half to even.
ON ERROR GOTO handler
PRINT "1. before any error:"; _ERRORLINE; _INCLERRORLINE; "["; _INCLERRORFILE$; "] ["; _ERRORMESSAGE$; "]"
ERROR 5
PRINT "2. after RESUME NEXT:"; _ERRORLINE

PRINT "3."
ERROR 6
x = 1: ERROR 7: y = 2
' a comment

a = 1: b = 0: ERROR 8
s
z = f(3)
PRINT "4. back in main:"; _ERRORLINE; ERR
PRINT "5. messages: ["; _ERRORMESSAGE$(5); "] ["; _ERRORMESSAGE$(0); "] ["; _ERRORMESSAGE$(1000); "] ["; _ERRORMESSAGE$(-1); "]"
DIM e AS LONG: e = 11
PRINT "6. ["; _ERRORMESSAGE$(e); "] ["; _ERRORMESSAGE$(9.6); "] ["; _ERRORMESSAGE$(52.5); "]"
ERROR 255
PRINT "7. continued:"
ERROR _
12
$IF WIN THEN
    ERROR 13
$END IF
PRINT "8. arithmetic:"; _ERRORLINE + 0.5; _ERRORLINE \ 2
ERROR 14: x = 1 + _
2
x = 1 + _
2: ERROR 15
ERROR _
_
16
IF x = 3 THEN _
ERROR 17
IF x = _
3 THEN
    ERROR 18
END IF
GOSUB g
PRINT "9. after the GOSUB:"; _ERRORLINE
SYSTEM

g:
ERROR 19
RETURN

handler:
PRINT "  error"; ERR; "_ERRORLINE"; _ERRORLINE; "_INCLERRORLINE"; _INCLERRORLINE; "file ["; _INCLERRORFILE$; "] message ["; _ERRORMESSAGE$; "]"
RESUME NEXT

SUB s
    PRINT "  in s"
    ERROR 9
END SUB

FUNCTION f (n)
    ERROR 10
    f = n
END FUNCTION
