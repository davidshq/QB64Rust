$Console:Only
' Storing into an integer variable (numeric-semantics spec): half to even, SINGLE narrowing for INTEGER targets,
' truncation to the target width.
x% = 2.5: Print x%
x% = 3.5: Print x%
x% = -2.5: Print x%
x% = 0.5: Print x%
x% = 1.5: Print x%
l& = 2.5: Print l&
l& = 3.5: Print l&
q&& = 2.5#: Print q&&
q&& = -3.5: Print q&&
d# = 2.5000001: x% = d#: l& = d#
Print x%; l&
d# = 16777217.5#: l& = d#: Print l&
s! = 2.5: x% = s!: Print x%
x% = 32767.6: Print x%
x% = 98765.4: Print x%
l& = 3000000000#: Print l&
s! = 0.1#: Print s!
s! = 1.23456789#: Print s!
d# = s!: Print d#
s! = 7: Print s!
d# = 9223372036854775807: Print d#
End
