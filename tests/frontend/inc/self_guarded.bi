' Includes itself under a $IF that its own $LET turns off (verification\v16_m7_self_guarded): parsed twice, the
' second time inactive.
$IF DONE = UNDEFINED THEN
$LET DONE = 1
'$INCLUDE:'self_guarded.bi'
CONST GUARDK = 1
$END IF
