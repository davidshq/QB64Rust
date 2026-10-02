$CONSOLE:ONLY
' Verification: label before TYPE on the same line.
lbl3: TYPE MyType
    Field1 AS INTEGER
END TYPE
DIM v AS MyType
v.Field1 = 3
PRINT v.Field1
SYSTEM
