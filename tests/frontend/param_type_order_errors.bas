' TEST: check-fail
' A parameter's TYPE must be defined above its SUB or FUNCTION (m4-builtins-param-types design D5; verification/
' v48_x12, x25, v49_x60-x65): a TYPE below at the top level, in the procedure's own body and in a procedure below,
' for a scalar and an array parameter, a SUB and a FUNCTION, called or not. A TYPE above, in a SUB or an IF block,
' is fine (v49_f, x66), and every TYPE serves DIM ... AS wherever it stands.
$CONSOLE:ONLY
IF 1 THEN
    TYPE early
        k AS LONG
    END TYPE
END IF
DIM v AS late
SYSTEM
SUB fine (p AS early, q() AS early)
END SUB
SUB below (p AS late)
END SUB
SUB below_array (p() AS late)
END SUB
FUNCTION below_function (p AS late)
END FUNCTION
SUB own_body (p AS own)
    TYPE own
        k AS LONG
    END TYPE
END SUB
SUB in_later (p AS later_t)
END SUB
TYPE late
    k AS LONG
END TYPE
FUNCTION defines
    TYPE later_t
        k AS LONG
    END TYPE
END FUNCTION
SUB fine_after (p AS late, q AS own)
END SUB
