' Signed 64-bit overflow in generated C++: does _INTEGER64 depend on the C++ optimisation level like LONG (v11)?
' Default build: run.sh. Optimised build (recorded by hand in v12_wrap_int64.O2.out.txt):
'   qb64pe.exe -x -q -m -f:OptimizeCppProgram=true v12_wrap_int64.bas -o v12_wrap_int64_O2.exe
$CONSOLE:ONLY
DIM x AS _INTEGER64, k AS LONG
x = 1
FOR k = 1 TO VAL("62") ' VAL keeps the C++ compiler from folding the value; a DOUBLE cannot hold 2^63 - 1
    x = x * 2
NEXT
x = (x - 1) + x ' 9223372036854775807
PRINT x
IF x + 1 > x THEN PRINT "x + 1 > x : true" ELSE PRINT "x + 1 > x : false"
PRINT x + 1
DIM y AS _INTEGER64
y = x + 1
PRINT y
SYSTEM
