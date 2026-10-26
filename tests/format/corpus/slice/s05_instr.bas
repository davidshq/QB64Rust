$Console:Only
' INSTR with and without its optional start argument. A start of 0 was put last in case it raised an error (the
' rest of the PRINT skipped, the program stopped); measured 2026-10-03: it does not raise, it behaves like 1.
s$ = "hello world"
Print InStr(s$, "o")
Print InStr(6, s$, "o")
Print InStr(s$, "")
Print InStr(5, s$, "")
Print InStr(20, s$, "o")
Print InStr(2.6, s$, "o")
Print InStr(s$ + "!", "!")
Print InStr("", "")
Print InStr(1, "abc", "abcd")
Print "before"; InStr(0, s$, "o"); "after"
Print "not reached if INSTR(0, ...) raises"
End
