$console:only
' _ERRORLINE, _INCLERRORLINE, _INCLERRORFILE$ and _ERRORMESSAGE$ in the main file (m3-errorline-chr-const;
' verification\v43_a_errorline, v43_d_continued, v43_x44): before any error, after errors on several lines, after
' RESUME NEXT, in a SUB, a FUNCTION and a GOSUB, a line with several statements, blank lines and comments, lines
' continued with an underscore (a line joined to the next is numbered as the last line of the join), a line in a
' $IF branch, the message of codes given as an argument, rounded half to even.
on error goto handler
print "1. before any error:"; _errorline; _inclerrorline; "["; _inclerrorfile$; "] ["; _errormessage$; "]"
error 5
print "2. after RESUME NEXT:"; _errorline

print "3."
error 6
x = 1: error 7: y = 2
' a comment

a = 1: b = 0: error 8
s
z = f(3)
print "4. back in main:"; _errorline; err
print "5. messages: ["; _errormessage$(5); "] ["; _errormessage$(0); "] ["; _errormessage$(1000); "] ["; _errormessage$(-1); "]"
dim e as long: e = 12
print "6. ["; _errormessage$(e); "] ["; _errormessage$(9.6); "] ["; _errormessage$(52.5); "]"
error 255
print "7. continued:"
ERROR _
12
$IF WIN THEN
    error 13
$end if
print "8. arithmetic:"; _errorline + 0.5; _errorline \ 2
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
    error 18
end if
gosub g
print "9. after the GOSUB:"; _errorline
system

g:
error 19
return

handler:
print "  error"; err; "_ERRORLINE"; _errorline; "_INCLERRORLINE"; _inclerrorline; "file ["; _inclerrorfile$; "] message ["; _errormessage$; "]"
resume next

sub s
    print "  in s"
    error 9
end sub

function f (n)
    error 10
    f = n
end function
