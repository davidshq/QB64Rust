$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x21_member_type_later_block)
TYPE t1
    x AS t2
END TYPE
TYPE t2
    y AS LONG
END TYPE
DIM a AS t1
a.x.y = 5
PRINT a.x.y; LEN(a)
SYSTEM
