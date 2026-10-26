$Console:Only
' Slice (m4-qb64pe-builtins, task 6.1): _DIR$ (no path printed), _SHELLHIDE, _COMMANDCOUNT, the shape of
' _COMPILEDATE$ and _COMPILETIME$, and _READBIT (verification\v53_c_system).
d$ = _Dir$("fonts")
Print Right$(d$, 1) = "\"; _DirExists(d$); _Dir$("bogus") = _Dir$("desktop")
Print _ShellHide("cmd /c exit 3"); _ShellHide("cmd /c exit 2") * 1000000000000
Print _CommandCount
Print Len(_CompileDate$); Len(_CompileTime$); Mid$(_CompileTime$, 3, 1)
Print _ReadBit(5, 0); _ReadBit(5, 1); _ReadBit(5, 2)
i64&& = -9223372036854775808: Print _ReadBit(i64&&, 63); _ReadBit(i64&&, 62)
System
