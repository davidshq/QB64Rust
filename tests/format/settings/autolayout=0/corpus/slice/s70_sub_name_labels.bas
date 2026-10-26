DECLARE SUB topbar ()
$Console:Only
' Slice (m3-get-put-bload, task 2.2): a line that is the name of a SUB and a colon. With no jump to the name
' anywhere in the program it is a call of the SUB; with a jump in any body it is a label in every body, and CALL
' still calls the SUB; a SUB with a parameter and a FUNCTION are always labels (verification\v28_c_labels,
' v28_x71-x92). Civil War Strategy has the call (topbar:) and the label beside a CALL (ships:, cannon:).
Dim Shared n As Integer
Dim v As Integer
Call topbar
twice:
twice:
after: Print "after the call"
param:
fname:
Print "main"; fname
Call param(3)
GoSub part
Call navy
onesub:
Print "main goes on"
Call other
thenonly:
If n > 0 Then thenonly
' Each kind of jump makes its name a label.
GoSub viagosub
Restore viarestore
Read v
Print "read"; v
On Error GoTo viaerror
Error 5
Print "back"
On 2 GOTO part, viaon
Print "not printed"
viaon:
Print "via on"
System
part:
Print "part"
Return
topbar:
Print "not reached"
viagosub:
Print "label viagosub"
Return
Data 1
viarestore:
Data 2
viaerror:
Print "label viaerror"
Resume Next

Sub topbar
    Print "sub topbar"
End Sub

Sub twice
    Print "sub twice"
End Sub

Sub after
    Print "sub after"
End Sub

Sub param (k As Integer)
    Print "sub param"; k
End Sub

Function fname
    fname = 7
End Function

Sub navy
    ships:
    n = n + 1
    Print "navy"; n
    If n < 2 GoTo ships
    Call ships
    cannon:
    n = n + 1
    If n < 5 Then GoTo cannon
    Call cannon
    Print "navy ends"; n
End Sub

Sub ships
    Print "sub ships"
End Sub

Sub cannon
    Print "sub cannon"
End Sub

Sub onesub
    Print "sub onesub"
End Sub

Sub other
    Dim k As Integer
    onesub:
    k = k + 1
    If k < 2 Then GoTo onesub
    Print "other"; k
End Sub

Sub thenonly
    Print "sub thenonly"
End Sub

Sub viagosub
    Print "sub viagosub"
End Sub

Sub viarestore
    Print "sub viarestore"
End Sub

Sub viaerror
    Print "sub viaerror"
End Sub

Sub viaon
    Print "sub viaon"
End Sub
