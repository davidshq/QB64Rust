' TEST: check-fail
$CONSOLE:ONLY
' Labels per body (m2-control-flow-slice task 5.1, design D5), each error as measured (verification\v17_d_*)
GOTO inside
ON ERROR GOTO inside
' A label with a leading _ is an invalid name (verification\v50_x28); the jump to it gives no error of its own.
GOTO _under
SYSTEM
done:
_under:
SUB t
    a:
    a:
    GOTO done
    GOSUB done
    ON ERROR GOTO a
    RETURN a
    RESUME done
    inside:
END SUB
