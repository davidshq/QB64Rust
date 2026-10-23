$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x20_key_dim)
' key$ in DIM with another name (QB64pe ini.bm).
DIM section$, key$
key$ = "k": section$ = "s"
PRINT key$; section$
SYSTEM
