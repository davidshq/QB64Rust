' Signed 32-bit overflow in generated C++ is undefined behaviour: the result depends on the C++ optimisation level.
' Default build: run.sh. Optimised build (recorded by hand in v11_wrap_o2.O2.out.txt):
'   qb64pe.exe -x -q -m -f:OptimizeCppProgram=true v11_wrap_o2.bas -o v11_wrap_o2_O2.exe
$Console:Only
Dim x As Long
x = Val("2147483647") ' VAL keeps the C++ compiler from folding the value
If x + 1 > x Then Print "x + 1 > x : true" Else Print "x + 1 > x : false"
Print x + 1
Dim y As Long
y = x + 1
Print y
Dim i As Integer
i = Val("32767")
If i + 1 > i Then Print "i + 1 > i : true" Else Print "i + 1 > i : false"
Print i + 1
i = i + 1
Print i
System
