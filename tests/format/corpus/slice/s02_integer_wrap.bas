$Console:Only
' Integer arithmetic width and wrap (numeric-semantics spec; DIVERGENCES.md D-001, D-002).
i% = 32767
Print i% + 1
i% = i% + 1
Print i%
x% = 70000
Print x%
c% = 200: d% = 200
Print c% * d%
e% = c% * d%
Print e%
Print 200 * 200
l& = 2147483647
Print l& + 1
l& = l& + 1
Print l&
Print 2147483647 * 2
Print 2147483647 + 1
Print 50000 * 50000
Print -2147483647 - 2
q&& = 9223372036854775807
Print q&&
Print q&& + 1
q&& = q&& + 1
Print q&&
Print 2147483648 * 2
Print 3037000500 * 3037000500
l& = 50000: Print l& * l&; l& * 50000&&
x% = -32768: Print -x%; -(-32768)
l& = -2147483647 - 1: Print -l&
End
