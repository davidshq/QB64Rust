$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): the entry for names that start with an underscore: a name with
' two underscores under DEFINT A-Z, and after _DEFINE _ AS LONG. LEN of a variable shows its type: 2 INTEGER,
' 4 LONG or SINGLE, 8 DOUBLE.
DEFINT A-Z
__a = 2.5: PRINT "__a under DEFINT A-Z:"; __a; LEN(__a)
_DEFINE _ AS DOUBLE
__b = 2.5: PRINT "__b after _DEFINE _ AS DOUBLE:"; __b; LEN(__b)
SYSTEM
