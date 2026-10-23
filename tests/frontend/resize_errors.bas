' TEST: check-fail
$CONSOLE:ONLY
' The forms of $RESIZE the old compiler rejects with "Syntax error" (m4-qb64pe-builtins design D3, task 4.1;
' verification\v53_x70, x71, x73-x75, x79, x81): alone, another word, a blank after or before the colon, a word or a
' comment after, a second colon. None is "not supported yet"; the messages are new.
$RESIZE
$RESIZE:BOGUS
$RESIZE: ON
$RESIZE :ON
$RESIZE:ON extra
$RESIZE:ON 'comment
$RESIZE:ON:ON
