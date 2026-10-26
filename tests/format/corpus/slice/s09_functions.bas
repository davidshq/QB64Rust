$Console:Only
' FUNCTION results (spec language/procedures, "Calls"): each result type, printed with the function's own type;
' a call before the definition; the result assigned with and without the suffix; EXIT FUNCTION before any
' assignment (0 or ""); zero-argument functions; a function changing its argument inside a PRINT (by reference
' for n&, a copy for the SINGLE n).
Print "before definition:"; twice&(3)
Print twice&(3) + 1; twice&(twice&(2))
Print fi%(7); fl&(7); fq&&(7)
Print fs!(7); fd#(7); ff##(7)
Print nosuffix(7)
Print "["; fstr$("ab"); "]"
Print "rounded:"; round%(2.5); round%(3.5); round%(-2.5)
Print "assigned without suffix:"; noassign&(4)
Print "exit early:"; early&(1); "["; earlys$; "]"
Print "zero args:"; zero&; zero& + 1
n& = 1
Print incr&(n&); n&
n = 1
Print incr&(n); n
Print "x"; fstr$("y"); "z"
System

Function twice& (a As Long)
    twice& = a * 2
End Function

Function fi% (a As Long)
    fi% = a * 1000
End Function

Function fl& (a As Long)
    fl& = a * 100000
End Function

Function fq&& (a As Long)
    fq&& = a * 10000000000&&
End Function

Function fs! (a As Long)
    fs! = a / 3
End Function

Function fd# (a As Long)
    fd# = a / 3
End Function

Function ff## (a As Long)
    ff## = a / 3
End Function

Function nosuffix (a As Long)
    nosuffix = a / 3
End Function

Function fstr$ (t As String)
    fstr$ = t + t
End Function

Function round% (v As Double)
    round% = v
End Function

Function noassign& (a As Long)
    noassign = a + 1
End Function

Function early& (a As Long)
    Exit Function
    early& = 5
End Function

Function earlys$
    Exit Function
    earlys$ = "x"
End Function

Function zero&
    zero& = 42
End Function

Function incr& (v As Long)
    v = v + 1
    incr& = v * 10
End Function
