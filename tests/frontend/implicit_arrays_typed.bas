' TEST: typed
$CONSOLE:ONLY
' Arrays used without DIM (m2-defxxx-common-arrays task 5.1, design D5; verification\v24_f_*, v24_x108, x109, x113,
' x258 to x260, x263). In the main module a static array, OPTION BASE to 10 per index written, of the suffix's type
' or the default type at that line; LBOUND and UBOUND of a name that is no array make one of one dimension; a read
' may be the first use. A scalar of the name, typed with AS or not, and a CONST of the name are other things; a
' type suffix that the declared array of the name does not have is another array. A built-in's name without its
' $ and a name with two underscores are free. ERASE of one clears it.
z(10) = 4
m(2, 3) = z(1)
s$(2) = "ab"
q&&(10) = 5
PRINT r(4); LBOUND(lb); UBOUND(ub)
DIM x AS LONG
x = 7
x(3) = 2.5
CONST k = 1
k(1) = k
DIM c(3) AS LONG
c!(7) = 1.5
c(1) = 2
chr(1) = 5
PRINT left(3); __foo(1)
ERASE z, s$
' The default type and OPTION BASE as they stand at the first use.
DEFINT A-Z
OPTION BASE 1
d(1) = 2.5
e(1, 2) = d(1)
DEFSNG A-Z
OPTION BASE 0
' Below $DYNAMIC the array is dynamic, and the statement that names it first allocates it before anything else
' it does (v24_x88, x101): a use, a second use, LBOUND, two arrays in one statement, a one-line IF's statement.
'$DYNAMIC
dy(3) = dy(3) + 1
dy(4) = 2
PRINT LBOUND(dl)
t1(1) = t2(1, 1)
IF x THEN ti(1) = 2
'$STATIC
st(1) = 1
p 4
SYSTEM
' In a procedure it is a local dynamic array that nothing allocates (every use raises error 9, as in the old
' compiler): also beside a main-module array that is not shared, a DIM SHARED scalar typed with AS and a
' parameter of the name. ERASE of one releases it.
DIM SHARED g AS LONG
SUB p (n)
    a(1) = z(1)
    g(1) = 2.5
    n(1) = n
    PRINT UBOUND(a); LBOUND(b, 1)
    ERASE a
END SUB
