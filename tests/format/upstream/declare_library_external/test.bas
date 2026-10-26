$Console:Only

$If WIN Then

    ' Windows can't use regular DECLARE LIBRARY to link against a dll, just skip this test
    Print 5

$Else

    $If 32BIT Then
    DECLARE LIBRARY "./lib32"
        FUNCTION add_values&(BYVAL v1 AS LONG, BYVAL v2 as LONG)
    END DECLARE
    $Else
    DECLARE LIBRARY "./lib"
        FUNCTION add_values&(BYVAL v1 AS LONG, BYVAL v2 as LONG)
    END DECLARE
    $End If

result = add_values&(2, 3)
PRINT result

$End If

System
