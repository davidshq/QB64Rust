$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): _SETBIT, _RESETBIT, _TOGGLEBIT of a literal and of each integer type, a bit
' beyond the width of the type and of 64 bits, a negative bit, float arguments, and the result stored and in HEX$.
PRINT _SETBIT(5, 1); _RESETBIT(5, 0); _TOGGLEBIT(5, 2)
b%% = 1: PRINT _SETBIT(b%%, 7); _RESETBIT(b%%, 0); _TOGGLEBIT(b%%, 7); _SETBIT(b%%, 8)
ub~%% = 1: PRINT _SETBIT(ub~%%, 7); _TOGGLEBIT(ub~%%, 7); _SETBIT(ub~%%, 8)
i% = -2: PRINT _SETBIT(i%, 0); _RESETBIT(i%, 15); _TOGGLEBIT(i%, 15); _SETBIT(i%, 16)
ui~% = 0: PRINT _SETBIT(ui~%, 15); _SETBIT(ui~%, 16)
l& = 0: PRINT _SETBIT(l&, 31); _SETBIT(l&, 30); _SETBIT(l&, 32)
ul~& = 0: PRINT _SETBIT(ul~&, 31)
q&& = 0: PRINT _SETBIT(q&&, 63); _SETBIT(q&&, 62); _TOGGLEBIT(q&&, 63)
uq~&& = 0: PRINT _SETBIT(uq~&&, 63)
o%& = 0: PRINT _SETBIT(o%&, 40)
n& = 40: PRINT _SETBIT(5, n&); _SETBIT(5, 40)
n& = 64: PRINT _SETBIT(5, n&); _RESETBIT(5, n&); _TOGGLEBIT(5, n&)
n& = -1: PRINT _SETBIT(5, n&); _TOGGLEBIT(0, n&)
PRINT _SETBIT(5.7, 1); _SETBIT(5, 1.6); _RESETBIT(7.5, 0); _TOGGLEBIT(-1.5, 0)
s! = 5.7: PRINT _SETBIT(s!, 1)
x&& = _SETBIT(1, 0): PRINT x&&
y& = _TOGGLEBIT(1, 0): PRINT y&
PRINT HEX$(_SETBIT(0, 63)); " "; HEX$(_TOGGLEBIT(-1, 0)); " "; HEX$(_RESETBIT(-1, 63))
SYSTEM
