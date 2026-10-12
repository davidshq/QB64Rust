$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (105_swap_offset_integer64)
DIM o AS _OFFSET, q AS _INTEGER64
o = 1: q = 2
SWAP o, q
PRINT o; q
SYSTEM
