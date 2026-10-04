$CONSOLE:ONLY
' Verification: DIM a AS T is an error ("Name already in use") once an earlier DIM a AS ... typed the plain name,
' even with another type. (The same type twice: v13b.)
DIM x AS INTEGER
DIM x AS LONG
PRINT x
SYSTEM
