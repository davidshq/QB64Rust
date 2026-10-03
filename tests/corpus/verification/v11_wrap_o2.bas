' Signed 32-bit overflow in generated C++ is undefined behaviour: the result depends on the C++ optimisation level.
' Default build: run.sh. Optimised build (recorded by hand in v11_wrap_o2.O2.out.txt):
'   qb64pe.exe -x -q -m -f:OptimizeCppProgram=true v11_wrap_o2.bas -o v11_wrap_o2_O2.exe
$CONSOLE:ONLY
DIM x AS LONG
x = VAL("2147483647") ' VAL keeps the C++ compiler from folding the value
IF x + 1 > x THEN PRINT "x + 1 > x : true" ELSE PRINT "x + 1 > x : false"
PRINT x + 1
DIM y AS LONG
y = x + 1
PRINT y
DIM i AS INTEGER
i = VAL("32767")
IF i + 1 > i THEN PRINT "i + 1 > i : true" ELSE PRINT "i + 1 > i : false"
PRINT i + 1
i = i + 1
PRINT i
SYSTEM
