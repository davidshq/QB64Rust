$CONSOLE:ONLY
' Verification: what a type suffix means on a name that DIM declared (design D5 of m2-workspace-and-slice).
' A variable is a name plus a type. DIM a AS T makes the plain name mean type T from then on, so a<suffix of T> is
' the same variable; a name with any other suffix is a separate variable. DIM a% declares only a%.
' Error cases: v13b_dim_suffix_error.bas, v13c_dim_retype_error.bas, v13d_dim_implicit_error.bas.
DIM a AS LONG
a = 5: a& = a& + 1
PRINT a; a&
DIM b AS LONG
b = 5: b! = 2.5: b% = 7
PRINT b; b!; b%
c! = 2.5
DIM c AS LONG
PRINT c; c!
d = 1.5
PRINT d; d!
e = 2.5
DIM e AS LONG
PRINT e; e!
DIM f AS STRING
f$ = "s": f% = 3
PRINT f; f$; f%
DIM g%
g = 2.5
PRINT g; g%
DIM h AS SINGLE
h = 1.5
PRINT h; h!
DIM i%
DIM i AS LONG
i = 4: i% = 5
PRINT i; i%
' A plain DIM after DIM AS changes nothing; DIM AS after a plain DIM is accepted and retypes the plain name.
DIM m AS LONG
DIM m
m = 2.5
PRINT m; m&; m!
DIM n
DIM n AS LONG
n = 2.5
PRINT n; n!
SYSTEM
