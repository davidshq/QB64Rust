$Console:Only
' Literal typing (numeric-semantics spec): each literal form printed; the type shows in the format.
Print "["; 5; -3; 1.5; "]"
Print -3
Print 40000
Print 32767; -32768; 32768; -32769
Print 2147483647; 2147483648; -2147483647; -2147483648
Print 5%; 5&; 5&&; -5%
Print 1.5E2; 2.5; .5; 1.; 1E5; 1D5
Print 1.5!; 1.5#; 1.5##; 1.5F0
Print 0.1; 0.1!; 0.1#
Print 1234567; 1234567.; 12345678.; 1.234567; 1.2345678; 1.23456789012345678
Print 3.14159265358979; 3.141592653589793; 3.1415926535897932
Print 1E-3; 2.5E+3; 1.5D-2; 7E38
Print &HFF; &HFFFF; &H8000; &H10000; &HFFFF&; &HFFFFFFFF; &H100000000
Print &O17; &O177777; &B101; &B1111111111111111
d# = 0.1!: Print d#
d# = 0.1: Print d#
s! = 0.1: d# = s!: Print d#
End
