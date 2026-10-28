$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 rows 17 and 18, a constant bit number or shift count outside 0 to 63
' (a C++ shift by it, which g++ folds), then the same numbers in a LONG variable (x86 masks them to 6 bits).
DIM n AS LONG
PRINT _SETBIT(0, 64); _SETBIT(0, 65); _READBIT(2, 65); _RESETBIT(-1, 64); _TOGGLEBIT(0, 64); _SETBIT(0, -1)
PRINT _SHL(1, 64); _SHL(1, 65); _SHR(2, 65); _SHR(1, 64); _SHL(1, -1)
n = 64: PRINT _SETBIT(0, n);
n = 65: PRINT _SETBIT(0, n); _READBIT(2, n);
n = 64: PRINT _RESETBIT(-1, n); _TOGGLEBIT(0, n);
n = -1: PRINT _SETBIT(0, n)
n = 64: PRINT _SHL(1, n);
n = 65: PRINT _SHL(1, n); _SHR(2, n);
n = 64: PRINT _SHR(1, n);
n = -1: PRINT _SHL(1, n)
SYSTEM
