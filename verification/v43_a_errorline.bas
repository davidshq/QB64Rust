$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): _ERRORLINE, _INCLERRORLINE, _INCLERRORFILE$ and
' _ERRORMESSAGE$ in the main file: before any error, after errors on several lines, beside ERL and a line
' number, after RESUME NEXT, in a SUB and a FUNCTION, a line with several statements, blank lines and comments,
' a statement continued over two lines, a line in a $IF branch, the message of codes given as an argument.
ON ERROR GOTO handler
PRINT "1. before any error:"; _ERRORLINE; _INCLERRORLINE; "["; _INCLERRORFILE$; "] ["; _ERRORMESSAGE$; "]"
ERROR 5
PRINT "2. after RESUME NEXT:"; _ERRORLINE

PRINT "3."
10 ERROR 6
x = 1: ERROR 7: y = 2
' a comment

20 a = 1: b = 0: ERROR 8
s
z = f(3)
PRINT "4. back in main:"; _ERRORLINE; ERR; ERL
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
ERROR 300
SYSTEM

handler:
PRINT "  error"; ERR; "ERL"; ERL; "_ERRORLINE"; _ERRORLINE; "_INCLERRORLINE"; _INCLERRORLINE; "file ["; _INCLERRORFILE$; "] message ["; _ERRORMESSAGE$; "]"
RESUME NEXT

SUB s
    PRINT "  in s"
    ERROR 9
END SUB

FUNCTION f (n)
    ERROR 10
    f = n
END FUNCTION
