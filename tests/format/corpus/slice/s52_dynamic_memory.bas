$Console:Only
' A dynamic array with fewer than no element (spec language/arrays-and-types, "Dynamic arrays", scenario "Fewer
' than no element"): an upper bound two or more below the lower bound is "Out of memory", which ends the program
' and which no handler receives; the line after the DIM does not run.
On Error GoTo h
Dim n As Long
n = -3
Print "DIM d(n) with n = -3:";
Dim d(n) As Long
Print " then UBOUND(d):";
Print UBound(d)
System
h:
Print " [error"; Err; "]";
Resume Next
