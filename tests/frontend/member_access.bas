' TEST: check-fail
$CONSOLE:ONLY
' Member access and omitted arguments parse, and sema marks each "not supported yet" (m2-parser-breadth task 5.1;
' since m2-defxxx-common-arrays task 5.1, `a(...)` without `DIM` is an array of numbers, whose element has no
' member: "Invalid expression" in the old compiler, verification\v18_h_member_of_numeric_element, also where an
' index follows the member; since m3-images-fonts-mouse group 2, a function's slot alone in brackets may be left
' empty before an argument, `_LOADIMAGE(f$, , r$)`: a gap at the end, at the start or in a required slot keeps its
' mark, and a gap with too many arguments is the arity error)
PRINT a(1).b
PRINT a(2) .b.c(3)
a(1).b = 5
a(1) = 2
x = INSTR(, "ab", "b")
x = _LOADIMAGE("a", )
x = _LOADIMAGE(, 32)
x = _RGB(1, , 3)
x = _LOADIMAGE("a", , , 4)
PRINT a.b
s (5 / 2) = 2, 0
SUB s (c AS LONG, d AS LONG)
END SUB
