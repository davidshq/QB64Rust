' Slice (m2-console-screenhide, group 3): _DEST, _SOURCE, _CONSOLE and _ECHO in a program with a console and a
' hidden window. Only what is printed while the console is the destination reaches standard output; _ECHO always
' does. No PRINT with a comma (tests\corpus\README.md).
$Console
$ScreenHide
Dim b As _Byte, ub As _Unsigned _Byte, i As Integer, l As Long, q As _Integer64, sg As Single, db As Double
Dim fl As _Float
On Error GoTo handler

' Switching the destination, and the handles.
_Echo "echo 1"
Print "window 1"
d& = _Dest
s& = _Source
_Dest _Console
Print "console 1"
Print "before:"; d&; s&
Print "now:"; _Dest; _Source; _Console
_Dest d&
Print "window 2"
_Echo "echo 2: " + Str$(_Dest)
_Dest _Console
Print "console 2"
_Dest 0
Print "window 3"
_Dest _Console
Print "console 3"
_Source _Console
Print "source:"; _Source
_Source s&
Print "source:"; _Source

' Set in a procedure, it stays in force.
_Dest 0
toConsole
Print "console after the SUB"
_Dest 0
x = f
Print "console after the FUNCTION"; x; _Source
_Source 0

' A handle that does not exist: error 258, the destination and the source stay.
_Dest -99
Print "after the bad _DEST"; _Dest
_Source -99
Print "after the bad _SOURCE"; _Source

' The handle from every numeric type: a float is rounded, a wider integer narrowed.
c& = _Console
b = c&: i = c&: l = c&: q = c&: sg = c&: db = c&: fl = c&
_Dest 0: _Dest b: Print "byte"
_Dest 0: _Dest i: Print "integer"
_Dest 0: _Dest l: Print "long"
_Dest 0: _Dest q: Print "integer64"
_Dest 0: _Dest sg: Print "single"
_Dest 0: _Dest db: Print "double"
_Dest 0: _Dest fl: Print "float"
sg = -9.6: db = -10.4: fl = -10.5
_Dest 0: _Dest sg: Print "single -9.6"
_Dest 0: _Dest db: Print "double -10.4"
_Dest 0: _Dest fl: Print "float -10.5"
_Dest 0: _Dest -9.5: Print "literal -9.5"
q = 4294967286
_Dest 0: _Dest q: Print "integer64 4294967286"
ub = 0
_Dest ub: Print "window: unsigned byte 0"
_Dest b + 0: Print "expression"

' After CALL, in parentheses, in expressions.
_Dest 0
Call _Dest(_Console)
Call _Source(_Console)
Call _Echo("echo 3")
Print "after CALL"; _Dest; _Source
_Source (0)
_Dest (_Console)
Print _Dest + _Source; _Console * 2; -_Dest; _Console / 4; Hex$(_Console)
If _Dest = _Console Then Print "the console is the destination"
' The source is the window, whose handle reads as -2: neither CASE is taken and nothing is printed.
Select Case _Source
    Case _Console: Print "the console is the source"
    Case 0: Print "the window is the source"
End Select
System

handler:
Print "error"; Err
Resume Next

Sub toConsole
    _Dest _Console
    Print "console in the SUB"
    _Echo "echo in the SUB"
End Sub

Function f
    _Dest _Console
    _Source _Console
    f = 7
End Function
