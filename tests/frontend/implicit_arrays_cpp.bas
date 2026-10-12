' TEST: cpp
$CONSOLE:ONLY
' Arrays used without DIM in the C++ (m2-defxxx-common-arrays task 5.1): the static array of the main module is
' set up in maindata.txt, 0 to 10; below $DYNAMIC the allocation stands in the do-block of the statement that
' names the array first, before its store; a procedure's array gets its descriptor in dataK.txt, "not allocated",
' on every call, and freeK.txt releases its block (none is ever made) and the _MEM lock of the descriptor, for a
' string array with its strings (as the old compiler writes it, read from verification\v24_x263).
z(10) = 4
'$DYNAMIC
dy(3) = dy(3) + 1
p
SYSTEM
SUB p
    a(1) = 5
    s$(2) = "ab"
    PRINT UBOUND(a)
END SUB
