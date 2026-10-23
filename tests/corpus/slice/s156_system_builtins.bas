$CONSOLE:ONLY
' Slice (m4-qb64pe-builtins, task 6.1): _DIR$ (no path printed), _SHELLHIDE, _COMMANDCOUNT, the shape of
' _COMPILEDATE$ and _COMPILETIME$, and _READBIT (verification\v53_c_system).
d$ = _DIR$("fonts")
PRINT RIGHT$(d$, 1) = "\"; _DIREXISTS(d$); _DIR$("bogus") = _DIR$("desktop")
PRINT _SHELLHIDE("cmd /c exit 3"); _SHELLHIDE("cmd /c exit 2") * 1000000000000
PRINT _COMMANDCOUNT
PRINT LEN(_COMPILEDATE$); LEN(_COMPILETIME$); MID$(_COMPILETIME$, 3, 1)
PRINT _READBIT(5, 0); _READBIT(5, 1); _READBIT(5, 2)
i64&& = -9223372036854775808: PRINT _READBIT(i64&&, 63); _READBIT(i64&&, 62)
SYSTEM
