' TEST: check-fail
$CONSOLE:ONLY
' Member access and omitted arguments parse, and sema marks each "not supported yet" (m2-parser-breadth task 5.1;
' since m2-arrays-and-types, `a(...)` without `DIM` is an implicit array and a member array is marked)
PRINT a(1).b
PRINT a(2) .b.c(3)
a(1).b = 5
a(1) = 2
x = INSTR(, "ab", "b")
PRINT a.b
s (5 / 2) = 2, 0
SUB s (c AS LONG, d AS LONG)
END SUB
