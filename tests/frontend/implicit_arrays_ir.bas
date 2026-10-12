' TEST: ir
$CONSOLE:ONLY
' Arrays used without DIM in the IR (m2-defxxx-common-arrays task 5.1): a static array of the main module with
' its bounds and no operation; below $DYNAMIC a dynamic array whose Allocate is the first operation of the
' statement that names it first (an error it raises is that statement's, verification\v24_x101), once per array;
' in a procedure a local dynamic array with no Allocate at all.
z(10) = 4
OPTION BASE 1
m(2, 3) = z(1)
'$DYNAMIC
FOR i = 1 TO 3
    dy(3) = dy(3) + 1
NEXT
t1(1) = t2(1, 1)
IF i THEN ti(1) = 2
PRINT dy(3)
p
SYSTEM
SUB p
    a(1) = 5
    PRINT UBOUND(a)
    ERASE a
END SUB
