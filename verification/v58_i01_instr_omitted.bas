$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): INSTR with its first argument left out (the old compiler compiles it, v57_o07)
' beside INSTR with a start of 1, 2 and 3, with variables, an empty search string, an empty string and no match.
PRINT INSTR(, "ab", "b"); INSTR(, "abab", "b"); INSTR(1, "abab", "b"); INSTR(2, "abab", "b"); INSTR(3, "abab", "b")
a$ = "hello world": b$ = "o"
PRINT INSTR(, a$, b$); INSTR(1, a$, b$); INSTR(, a$, ""); INSTR(, "", b$); INSTR(, a$, "zz"); INSTR(, "", "")
n = INSTR(, a$ + "o", b$ + "r"): PRINT n
f$ = "fixed": DIM fx AS STRING * 5: fx = f$
PRINT INSTR(, fx, "x"); INSTR(, "axbxc", fx)
SYSTEM
