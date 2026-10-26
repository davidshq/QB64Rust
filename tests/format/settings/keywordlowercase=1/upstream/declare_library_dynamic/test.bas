$console:only

$if 32BIT then
    DECLARE DYNAMIC LIBRARY "./lib32"
        FUNCTION add_values&(BYVAL v1 AS LONG, BYVAL v2 as LONG)
    END DECLARE
$else
    $if _ARM_ then
        DECLARE DYNAMIC LIBRARY "./libarm64"
            FUNCTION add_values& (BYVAL v1 AS LONG, BYVAL v2 AS LONG)
        END DECLARE
    $else
        declare dynamic library "./lib"
            function add_values& (byval v1 as long, byval v2 as long)
        end declare
    $end if
$end if

result = add_values&(2, 3)
print result
system
