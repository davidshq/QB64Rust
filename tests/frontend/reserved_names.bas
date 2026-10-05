' TEST: check-fail
$CONSOLE:ONLY
' Reserved names (m2-procedures-and-errors D3, verification\v14_res_*): keywords and built-ins written without a
' required suffix are taken whatever suffix the name carries; built-ins with their required suffix are taken too;
' no variable, SUB or parameter name may start with `_` (verification\v15_underscore_*)
DIM cls AS LONG
DIM err AS LONG
key = 1
len& = 1
left$ = "x"
SUB p (name AS STRING)
END SUB
SUB q (cls)
END SUB
FUNCTION len& (a AS LONG)
END FUNCTION
SUB timer
END SUB
_foo = 1
SUB _bar
END SUB
SUB r (_p)
END SUB
