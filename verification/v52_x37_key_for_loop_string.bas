$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x37_key_for_loop_string)
' key$ passed to a built-in and assigned from one.
key$ = UCASE$("ab")
PRINT LEFT$(key$, 1); INSTR(key$, "B")
SYSTEM
