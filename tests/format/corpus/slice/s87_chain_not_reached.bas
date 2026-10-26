$Console:Only
' Slice (m3-console-files, task 7.1): CHAIN, compiled and never run. A program prints a line and leaves; below its
' SYSTEM stands CHAIN in each form, as code that is compiled, linked and not reached: when it runs, libqb starts
' the compiler beside the program through cmd. The call each form compiles to is compared by tests\callsite.
Dim f As String, x As String * 8, n As Long
f = "next"
Print "CHAIN: compiled"
If Len(f) = 0 Then Chain f
System

Chain "next.bas"
Chain f
Chain x
Chain f + ".bas"
Chain ("next")
Call Chain(f)
If n Then Chain f Else Chain "other"
