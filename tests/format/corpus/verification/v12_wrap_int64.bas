' Signed 64-bit overflow in generated C++: does _INTEGER64 depend on the C++ optimisation level like LONG (v11)?
' Default build: run.sh. Optimised build (recorded by hand in v12_wrap_int64.O2.out.txt):
'   qb64pe.exe -x -q -m -f:OptimizeCppProgram=true v12_wrap_int64.bas -o v12_wrap_int64_O2.exe
$Console:Only
Dim x As _Integer64, k As Long
x = 1
For k = 1 To Val("62") ' VAL keeps the C++ compiler from folding the value; a DOUBLE cannot hold 2^63 - 1
    x = x * 2
Next
x = (x - 1) + x ' 9223372036854775807
Print x
If x + 1 > x Then Print "x + 1 > x : true" Else Print "x + 1 > x : false"
Print x + 1
Dim y As _Integer64
y = x + 1
Print y
System
