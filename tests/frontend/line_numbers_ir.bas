' TEST: ir
$CONSOLE:ONLY
' A line number is a label that sets `last_line` (m4-line-numbers-whole-arrays, design D2), with the old
' compiler's text: `010` is 10, `10.5` 10.5, `30#` 30, `020&` 020. `THEN 10 ELSE 20` and `IF c GOTO n` branch to it.
010 IF c THEN 10.5 ELSE 30#
10.5 IF c GOTO 020&
30# PRINT ERL
020& SYSTEM
