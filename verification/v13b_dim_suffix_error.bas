$CONSOLE:ONLY
' Verification: DIM a AS T is an error ("Name already in use") when the variable a<suffix of T> already exists,
' here created implicitly. The same error for DIM x AS LONG, x AS LONG (checked by hand, 2026-10-03).
x& = 3
DIM x AS LONG
PRINT x; x&
SYSTEM
