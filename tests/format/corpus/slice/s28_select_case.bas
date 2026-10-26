$Console:Only
' Slice program (m2-core-builtins, D7, D9): SELECT CASE and SELECT EVERYCASE as measured in verification\v20_h_select:
' when the selector is read, items converted to the selector's type, string items, IS with each operator, errors
' in the selector and in items (the handler prints ERR and resumes next), jumps out of and into a CASE body, and a
' recursive FUNCTION across a SELECT (DIVERGENCES-QB45.md Q-002: one static copy per SELECT). No PRINT comma.
Dim Shared v As Integer
Dim Shared a(3) As Integer
Type t
    m As Integer
End Type
Dim Shared u As t
On Error GoTo h

Print "-- selector read once"
Select Case noisy
    Case 1: Print "one"
    Case 2: Print "two"
    Case 3: Print "three"
End Select

Print "-- a plain variable is read at each test; an element and a member once"
v = 1
Select Case v
    Case setv: Print "case setv"
    Case 2: Print "case 2"
    Case Else: Print "else"
End Select
a(1) = 1
Select Case a(1)
    Case seta: Print "case seta"
    Case 2: Print "case 2"
    Case Else: Print "else"
End Select
u.m = 1
Select Case u.m
    Case setm: Print "case setm"
    Case 2: Print "case 2"
    Case Else: Print "else"
End Select

Print "-- items converted to the selector's type"
Dim i As Integer
i = 2
Select Case i
    Case 2.4: Print "2.4 matches 2"
    Case Else: Print "else"
End Select
Select Case i
    Case 1.5: Print "1.5 matches 2"
    Case Else: Print "else"
End Select
Select Case i
    Case 2.5 To 3: Print "2.5 TO 3 matches 2"
    Case Else: Print "else"
End Select
Select Case i
    Case Is > 1.9: Print "IS > 1.9"
    Case Else: Print "IS > 1.9 does not match 2"
End Select
Dim f As Single
f = 2.5
Select Case f
    Case 2: Print "2"
    Case 3: Print "3"
    Case 2 To 3: Print "2.5 in 2 TO 3"
    Case Else: Print "else"
End Select
Dim d As Double
d = 0.1
Select Case d
    Case 0.1: Print "0.1 matches DOUBLE 0.1"
    Case Else: Print "0.1 does not match DOUBLE 0.1"
End Select
f = 0.1
Select Case f
    Case 0.1#: Print "0.1# matches SINGLE 0.1"
    Case Else: Print "0.1# does not match SINGLE 0.1"
End Select
Dim q As _Integer64
q = 4294967298
Select Case q
    Case 2: Print "4294967298 matches 2"
    Case 4294967298: Print "4294967298"
End Select
Select Case i + 1
    Case 3: Print "i + 1 is 3"
End Select

Print "-- string selectors"
Dim s As String
s = "dog"
Select Case s
    Case "a" To "m": Print "a TO m"
    Case Else: Print "else"
End Select
Select Case s
    Case Is < "b": Print "IS < b"
    Case Is >= "dog": Print "IS >= dog"
End Select
Select Case s
    Case "cat", "dog": Print "cat, dog"
End Select
Select Case s + "s"
    Case "dogs": Print "dogs"
End Select
Select Case Left$(s, 1)
    Case "d": Print "d"
End Select

Print "-- IS with each operator (EVERYCASE), selector 5"
i = 5
Select EveryCase i
    Case Is = 5: Print "= 5"
    Case Is <> 5: Print "<> 5"
    Case Is < 6: Print "< 6"
    Case Is > 4: Print "> 4"
    Case Is <= 5: Print "<= 5"
    Case Is >= 6: Print ">= 6"
End Select

Print "-- items of each kind"
For k = 1 To 4
    If k = 1 Then x = 3
    If k = 2 Then x = 5
    If k = 3 Then x = 11
    If k = 4 Then x = 8
    Select Case x
        Case 1, 3: Print "first"
        Case 4 To 6: Print "second"
        Case Is > 10: Print "third"
        Case Else: Print "else"
    End Select
Next
Select Case 7
    Case 9 To 1: Print "9 TO 1 matched 7"
    Case Else: Print "9 TO 1 did not match 7"
End Select
Select Case 9
    Case 1: Print "one"
End Select
Print "no match, no ELSE"

Print "-- EVERYCASE"
x = 5
Select EveryCase x
    Case Is > 1: Print "> 1"
    Case Is > 2: Print "> 2"
    Case Else: Print "else"
End Select
Select EveryCase x
    Case Is > 9: Print "> 9"
    Case Else: Print "else"
End Select
Select EveryCase x
    Case 5: Print "5": x = 6
    Case 6: Print "6 after the body changed x"
End Select
everyp 1
everyp 3

Print "-- errors in the selector and in items"
Select Case Asc("")
    Case 0: Print "case 0"
    Case 1: Print "case 1"
    Case Else: Print "else"
End Select
Select EveryCase Asc("")
    Case 0: Print "case 0"
    Case Else: Print "else"
End Select
x = 5
Select Case x
    Case Asc(""): Print "case ASC(empty) for 5"
    Case Else: Print "else for 5"
End Select
Select Case x
    Case 1, Asc(""), 2: Print "case 1, ASC(empty), 2 for 5"
    Case Else: Print "else for 5 (list)"
End Select
Select Case x
    Case Asc("") To 9: Print "case ASC(empty) TO 9 for 5"
    Case Else: Print "else for 5 (range)"
End Select
Select Case x
    Case Is > Asc(""): Print "case IS > ASC(empty) for 5"
    Case Else: Print "else for 5 (IS)"
End Select
Select Case a(9)
    Case 0: Print "bad index: element 0's value"
    Case Else: Print "else"
End Select

Print "-- jumps"
x = 1
Select Case x
    Case 1
        Print "in case 1, GOTO out"
        GoTo out1
        Print "not printed"
    Case Else
        Print "else"
End Select
out1:
Print "out1"
GoTo inside
Select Case 1
    Case 2
        Print "case 2 head"
        inside:
        Print "inside case 2"
    Case 3
        Print "case 3"
End Select
Print "after jump in"
exits
Select Case 1
End Select
Print "empty SELECT ran"
For k = 1 To 3
    Select Case k
        Case 2: Exit For
    End Select
    Print "k ="; k
Next
Print "EXIT FOR from a CASE at k ="; k

Print "-- recursion (Q-002)"
Print "rec(1) ="; rec(1)
Print "rec(3) ="; rec(3)
System

h:
Print "[handler"; Err; "]"
Resume Next

Function noisy
    Print "noisy called"
    noisy = 3
End Function

Function setv
    v = 2
    setv = 0
End Function

Function seta
    a(1) = 2
    seta = 0
End Function

Function setm
    u.m = 2
    setm = 0
End Function

Sub exits
    Select Case 1
        Case 1
            Print "EXIT SUB from a CASE"
            Exit Sub
    End Select
    Print "not printed"
End Sub

Sub everyp (n)
    Select EveryCase n * 2
        Case 2: Print "everyp 2"
        Case Is < 5: Print "everyp < 5"
        Case Else: Print "everyp else"
    End Select
End Sub

Function rec (n)
    Select Case n * 10
        Case probe(n): rec = -1
        Case 10: rec = 1
        Case 20: rec = 2
        Case 30: rec = 3
        Case Else: rec = 99
    End Select
End Function

Function probe (n)
    If n = 1 Then dummy = rec(2)
    probe = -5
End Function
