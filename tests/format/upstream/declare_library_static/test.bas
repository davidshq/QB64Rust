$Console:Only

$If WIN Then
    $If 32BIT Then
        DECLARE STATIC LIBRARY "./lib-windows32"
            FUNCTION add_values&(BYVAL v1 AS LONG, BYVAL v2 as LONG)
        END DECLARE
    $Else
        $If _ARM_ Then
            DECLARE STATIC LIBRARY "./lib-windowsarm64"
                FUNCTION add_values& (BYVAL v1 AS LONG, BYVAL v2 AS LONG)
            END DECLARE
        $Else
            Declare Static Library "./lib-windows"
                Function add_values& (ByVal v1 As Long, ByVal v2 As Long)
            End Declare
        $End If
    $End If
$ElseIf MAC Then
    DECLARE STATIC LIBRARY "./lib-osx"
        FUNCTION add_values&(BYVAL v1 AS LONG, BYVAL v2 as LONG)
    END DECLARE
$ElseIf LINUX Then
    $If 32BIT Then
        DECLARE STATIC LIBRARY "./lib-linux32"
            FUNCTION add_values&(BYVAL v1 AS LONG, BYVAL v2 as LONG)
        END DECLARE
    $Else
        DECLARE STATIC LIBRARY "./lib-linux"
            FUNCTION add_values&(BYVAL v1 AS LONG, BYVAL v2 as LONG)
        END DECLARE
    $End If
$End If

result = add_values&(2, 3)
Print result
System
