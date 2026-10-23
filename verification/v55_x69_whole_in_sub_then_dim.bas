$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x69_whole_in_sub_then_dim)
s
SYSTEM
SUB s
    DIM x(3)
    PRINT LEN(x())
END SUB
