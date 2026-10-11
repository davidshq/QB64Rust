' TEST: check-fail
$CONSOLE:ONLY
' The compile errors of the requirements "Dynamic arrays", "REDIM" and "ERASE" (m2-defxxx-common-arrays task 4.1),
' each measured with the old compiler (verification\v24_x39 to x42, x50, x52 to x55, x57 to x63, v18_e_dim_twice).
' The lines between them are accepted.
n = 3
' Another type written with AS than the array was declared with: "Name already in use"; between STRING and
' STRING * n, and another length: "Type mismatch".
REDIM x(5) AS LONG
REDIM x(5) AS INTEGER
REDIM fs(5) AS STRING
REDIM fs(5) AS STRING * 4
REDIM f3(5) AS STRING * 3
REDIM f3(5) AS STRING * 5
' Another number of dimensions: "Cannot change the number of elements an array has!", for every statement.
REDIM m(2, 3)
REDIM m(4)
REDIM _PRESERVE m(4)
DIM m(n)
DIM dy(n)
DIM dy(n, n)
DIM st(5) AS LONG
REDIM st(1, 2) AS LONG
' The words after REDIM: "Invalid variable name". PRESERVE without its underscore is a name, so a second name
' after it is a syntax error ("DIM: Expected ,", v24_x255).
REDIM SHARED _PRESERVE a(3) AS LONG
REDIM PRESERVE x(5) AS LONG
' Constant bounds that give no array, in a REDIM, below $DYNAMIC, and in one dimension beside a variable bound:
' "Invalid array bounds".
REDIM neg(-2) AS LONG
REDIM rev(5 TO 1) AS LONG
'$DYNAMIC
DIM rev2(5 TO 1) AS LONG
'$STATIC
DIM half(n TO 2, -1)
DIM part(n, 3 TO 1)
' A lower bound that is not constant is not compared.
DIM fine(n TO 2, 1 TO 3)
' A second DIM with constant bounds of a static array: "Cannot redefine a static array!". Below $DYNAMIC it is a
' statement that runs.
DIM st(5) AS LONG
'$DYNAMIC
DIM st(5) AS LONG
'$STATIC
' ERASE of anything that is no declared array: "Undefined array passed to ERASE".
x1 = 1
ERASE x1
ERASE nothing
ERASE st()
ERASE st(1)
s$ = "abc"
ERASE s$
ERASE st%
' A suffix that names the array is accepted; ERASE without a name is a syntax error.
ERASE st&, st, x&
ERASE
SYSTEM
