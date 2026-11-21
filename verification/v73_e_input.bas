$CONSOLE:ONLY
' Verification (wasm-pointer-width, design D2): console INPUT into an _OFFSET and an _UNSIGNED _OFFSET
' (v73_e_input.stdin: 4321, then 77,88). libqb stores as many bytes as the type value says, so the variables
' declared around them must keep their values where a pointer is 4 bytes.
DIM a AS LONG, p AS _OFFSET, b AS LONG, u AS _UNSIGNED _OFFSET, c AS LONG
a = 111: b = 222: c = 333
INPUT p
PRINT a; p; b; c
INPUT u, p
PRINT a; u; p; b; c
SYSTEM
