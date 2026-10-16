$CONSOLE:ONLY
' Where a DECLARE LIBRARY block may stand and what it may name: two libraries in one statement, a
' version after the name, the same library twice, a block in an included file, in a SUB, in $IF.
DECLARE LIBRARY "v37_lib/v37h", "./v37c"
    FUNCTION h_l& (BYVAL v AS LONG)
END DECLARE
DECLARE LIBRARY "./v37o", "./v37cpp"
    FUNCTION cpp_half# (BYVAL v AS DOUBLE)
END DECLARE
DECLARE LIBRARY "v37_lib/v37h.2"
    FUNCTION h_i% (BYVAL v AS INTEGER)
END DECLARE
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_s! (BYVAL v AS SINGLE)
END DECLARE
'$INCLUDE: 'v37_inc/v37_lib.bi'
$IF WIN THEN
    DECLARE LIBRARY "v37_lib/v37h"
        FUNCTION h_d# (BYVAL v AS DOUBLE)
    END DECLARE
$ELSE
    DECLARE LIBRARY "nowhere"
        FUNCTION h_d# (BYVAL v AS DOUBLE)
    END DECLARE
$END IF

PRINT "two:"; h_l&(1); cpp_half#(3)
PRINT "version:"; h_i%(9)
PRINT "again:"; h_s!(0.5)
PRINT "include:"; inc_add&(4, 5)
PRINT "$if:"; h_d#(0.25)
inner
PRINT "after sub:"; h_ul~&(1)
SYSTEM

SUB inner
    DECLARE LIBRARY "v37_lib/v37h"
        FUNCTION h_ul~& (BYVAL v AS _UNSIGNED LONG)
    END DECLARE
    DECLARE DYNAMIC LIBRARY "./v37dll"
        FUNCTION d_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    END DECLARE
    PRINT "sub:"; h_ul~&(7); d_add&(1, 1)
END SUB
