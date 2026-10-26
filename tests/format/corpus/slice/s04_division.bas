$Console:Only
' Division (numeric-semantics spec): integer / integer in _FLOAT printed with 16 digits; otherwise the wider float.
Print 1 / 3
Print 2 / 3
Print 7 / 2
Print 10 / 4
d# = 2.5: Print d# / 3
s! = 2.5: Print s! / 3
Print 1! / 3
Print 2 / 3!
Print 2 / 3#
a% = 1: b% = 3
Print a% / b%
s! = a% / b%: Print s!
d# = a% / b%: Print d#
x% = 7 / 2: Print x%
l& = 2147483647: Print l& / 2
q&& = 9223372036854775807: Print q&& / 2
Print 1 / 3 * 3
Print (1 + 2) / (4 - 1)
End
