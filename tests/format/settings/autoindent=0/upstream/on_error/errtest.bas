$Console:Only

'interfacing internal variables
Declare Library "errtest"
    Function GetErrorHistory$
    Function GetErrorGotoLine&
End Declare

'testing handler behavior
Status
On Error GoTo mainHandler 'set w/o history entry
Status
Error 5
NormalTest
Error 5
RecursiveTest 3
Error 5
On Error GoTo _LastHandler 'no more history entries => silent fallback to "unhandled"
Status

'return to test script
System

'-----------------------------------------------------------------------
mainHandler:
Print " CALLED: main handler"
Resume Next
'-----
nSubHandler:
Print " CALLED: normal sub handler"
Resume Next
'-----
rSubHandler:
Print " CALLED: recursive sub handler"
Resume Next

'-----------------------------------------------------------------------
Sub NormalTest
    On Error GoTo _NewHandler nSubHandler 'set with history entry
    Status
    Error 5
    On Error GoTo _LastHandler 'restore to the last handler from history
    Status
End Sub
'-----
Sub RecursiveTest (n%)
    On Error GoTo _NewHandler rSubHandler 'set with history entry
    Status
    Error 5
    If n% > 1 Then RecursiveTest n% - 1
    On Error GoTo _LastHandler 'restore to the last handler from history
    Status
End Sub

'-----------------------------------------------------------------------
Sub Status
    Print: Print "HISTORY: "; "<"; GetErrorHistory$; ">"
    Select Case GetErrorGotoLine&
        Case 0: Print " ACTIVE: unhandled"
        Case 1: Print " ACTIVE: main handler"
        Case 2: Print " ACTIVE: normal sub handler"
        Case 3: Print " ACTIVE: recursive sub handler"
    End Select
End Sub

