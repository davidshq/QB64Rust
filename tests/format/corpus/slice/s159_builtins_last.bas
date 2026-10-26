$Console:Only
' Slice (m4-builtins-remaining, task 3.1): the built-ins no program used before, each that a console program can
' run (verification\v58_f01-f13, s01-s04, s06b, i01): the hyperbolic and gradian functions, the bit functions, the
' MBF conversions, _COMPILERVERSION$, _ENVIRONCOUNT, _UPTIME, LPOS, the lock keys (0 with no window), _BLINK as a
' function and a statement, INP, OUT and WAIT on the ports libqb
' emulates and one it does not, DATE$ = and TIME$ = (nothing changes), the console statements, INSTR with its start
' left out, FILES of the folder's a.txt (the path and the free space masked by the .normalize rules), INPUT$ on
' that file (one character, none, past its end: error 62), and in code
' that is not reached the built-ins never run (design D3) and _CINP and _CONSOLEINPUT, which read the runner's
' console (a key event waiting there changed a recording).
On Error GoTo handler
Print _Asinh(1); _Acosh(2); _Atanh(.5)
s! = 1.5: d# = 1.5: f## = 1.5: i% = 2
Print _Asinh(s!); _Acosh(d#); _Atanh(f## / 3); _Asinh(i%)
Print _D2G(90); _G2D(100); _R2G(1); _G2R(200); _D2G(s!)
Print _SetBit(5, 1); _ResetBit(5, 0); _ToggleBit(5, 2)
b%% = 1: q&& = 0: ub~%% = 1: Print _SetBit(b%%, 7); _ToggleBit(q&&, 63); _ResetBit(ub~%%, 0); _SetBit(5, 40)
Print CVSMBF(MKSMBF$(1.5)); CVDMBF(MKDMBF$(-2.25#)); Len(MKSMBF$(1)); Len(MKDMBF$(1))
Print Asc(MKSMBF$(1.5), 4); Asc(MKDMBF$(1.5#), 8)
x! = CVSMBF("ab"): Print x!
Print Len(_CompilerVersion$) > 0; Left$(_CompilerVersion$, 8)
Print _EnvironCount > 0; _Uptime > 0; LPos(0)
x = LPos(4): Print x
Print _CapsLock; _NumLock; _ScrollLock
Print _Blink
_Blink Off: Print _Blink
_Blink On
Out &H3C0, 0: Print _Blink
k = Inp(&H60): u = Inp(&H379): Out &H378, 1: Print k = Inp(&H60); u
Wait &H378, 1
x = Inp(70000): Print x
d$ = Date$: Date$ = "01-02-2003": Print Date$ = d$
t$ = Left$(Time$, 5): Time$ = "12:34:56": Print Left$(Time$, 5) >= t$ Or t$ = "23:59"
_ConsoleTitle "slice": _ConsoleCursor _Hide: _ConsoleCursor _Show , 50: _ConsoleCursor
Print InStr(, "abab", "b"); InStr(, "hello", "zz"); InStr(2, "abab", "b")
Files "*.txt"
Open "a.txt" For Input As #1
Print Input$(1, # 1); "|"; Len(Input$(0, 1)); "|"
s$ = Input$(5, # 1): Print Len(s$)
Close #1
If k = 1 Then
    Print _ConsoleInput; _CInp; _CInp(0); _CInp(1)
    _ScreenClick 10, 20: _ScreenPrint "abc": _NotifyPopup "t", "m"
    c~& = _ColorChooserDialog: a$ = _InputBox$("t")
    _CapsLock On: _Numlock Off: _Scrolllock _Toggle
    _ConsoleFont "Consolas", 16
End If
Print "done"
System
handler:
Print "error"; Err
Resume Next
