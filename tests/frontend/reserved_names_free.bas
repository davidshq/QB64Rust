' TEST: check-ok
$CONSOLE:ONLY
' Names that are free (m2-procedures-and-errors D3, verification\v14_res_param_left, v14_res_var_left,
' v14_res_sub_chr): a built-in that must be written with `$` leaves the bare name free; WIDTH is free although
' it has no required suffix (v15_width_var); `_` inside a name is fine
left = 1
chr (left)
SUB chr (left AS LONG)
    PRINT left
END SUB
width = 80
a_b = width
