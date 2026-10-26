$Console:Only

$If 32BIT Then
    DECLARE DYNAMIC LIBRARY "./lib32"
        FUNCTION add_values&(BYVAL v1 AS LONG, BYVAL v2 as LONG)
    END DECLARE
$Else
  $If _ARM_ Then
        DECLARE DYNAMIC LIBRARY "./libarm64"
            FUNCTION add_values& (BYVAL v1 AS LONG, BYVAL v2 AS LONG)
        END DECLARE
  $Else
    Declare Dynamic Library "./lib"
      Function add_values& (ByVal v1 As Long, ByVal v2 As Long)
    End Declare
  $End If
$End If

result = add_values&(2, 3)
Print result
System
