$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): _ASINH, _ACOSH, _ATANH of literals and of a variable of each type (the digits
' printed show the result's type), and arguments out of their domain (the old compiler calls std::asinh and kin
' directly, so no error is expected, only the C library's value).
ON ERROR GOTO handler
PRINT _ASINH(1); _ACOSH(2); _ATANH(.5)
PRINT _ASINH(1.5); _ACOSH(1.5); _ATANH(.25)
s! = 1.5: d# = 1.5: f## = 1.5: i% = 2: l& = 3: q&& = 4: u~%% = 5
PRINT _ASINH(s!); _ASINH(d#); _ASINH(f##); _ASINH(i%); _ASINH(l&); _ASINH(q&&); _ASINH(u~%%)
PRINT _ACOSH(s!); _ACOSH(d#); _ACOSH(f##); _ACOSH(i%)
s! = .5: d# = .5: f## = .5
PRINT _ATANH(s!); _ATANH(d#); _ATANH(f##); _ATANH(0)
PRINT _ASINH(-3); _ASINH(0); _ACOSH(1)
x## = _ACOSH(0): PRINT x##
x## = _ACOSH(.5): PRINT x##
x## = _ATANH(1): PRINT x##
x## = _ATANH(-1): PRINT x##
x## = _ATANH(2): PRINT x##
x## = _ASINH(1E300): PRINT x##
y& = _ASINH(10): PRINT y&
SYSTEM
handler:
PRINT "error"; ERR
RESUME NEXT
