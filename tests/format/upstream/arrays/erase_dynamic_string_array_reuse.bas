$Console:Only
Option _Explicit

' Exercises the free/realloc/init cycle for a dynamic array of
' variable-length strings, where each element is one qbs* slot.

ReDim a(1 To 3) As String
a(1) = "red"
a(2) = "green"
a(3) = "blue"
AssertString "before erase", a(2), "green"

ReDim a(1 To 2) As String
AssertString "element after redim", a(1), ""

a(1) = "again"
a(2) = "once more"
AssertString "assign after redim", a(1), "again"
AssertString "assign after redim 2", a(2), "once more"

Erase a
ReDim a(1 To 1) ' We can't check for uninitialized arrays, so a REDIM after the ERASE is necessary
AssertString "erase/redim clears array contents", a(1), ""

ReDim a(0 To 0) As String
a(0) = "single"
AssertString "single element", a(0), "single"

ReDim b(1 To 4) As String
b(1) = "one"
b(4) = "four"
ReDim _Preserve b(1 To 6) As String
AssertString "preserve keeps first", b(1), "one"
AssertString "preserve keeps last", b(4), "four"
AssertString "preserve clears grown", b(6), ""

b(6) = "six"
AssertString "grown element usable", b(6), "six"

ReDim _Preserve b(1 To 2) As String
AssertString "shrink keeps first", b(1), "one"

System

'$Include:'../utilities/assert.bm'
