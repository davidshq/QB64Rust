' TEST: check-fail
' A built-in statement that has a template, after CALL, in a form the old compiler rejects (m3-sound task 2.1;
' qb64pe.bas 10617-10627; verification\v27_x14, x15, v28_x67, v29_x88, x90-x92, x94, x95, x97). After the name
' comes nothing, or one pair of parentheses around all that follows, with something in it ("Expected CALL sub-name
' (...)" otherwise); what stands between them must match a form of the statement's template ("Syntax error -
' Reference: ..."). The last line is a form that matches and has a wrong argument
DIM l AS LONG, t AS STRING
CALL PSET(1, 2)
CALL VIEW PRINT
CALL DEF SEG
CALL CLS()
CALL SOUND(440)
CALL SOUND(440, 1) WAIT
CALL PLAY()
CALL PLAY
CALL LINE((1, 2)-(3, 4)), 5
CALL LOCATE 1, 2
CALL PLAY(0)
SYSTEM
