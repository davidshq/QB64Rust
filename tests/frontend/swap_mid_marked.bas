' TEST: check-fail
$CONSOLE:ONLY
' Forms of SWAP and the MID$ statement the old compiler accepts that are "not supported yet", each marked
' at the form (m2-builtin-statements tasks 5.3, 5.4; verification\v22_x103): an operand in parentheses; SWAP
' through CALL (verification\v26_x128). Two whole arrays compile since m4-line-numbers-whole-arrays; a member of an
' element compiles as an operand of SWAP since m2-defxxx-common-arrays task 9.1 (member_arrays_typed.bas) and as the
' target of MID$ since m3-typefields task 4.2. RANDOMIZE through CALL compiles since m3-sound
' (call_template_ir.bas)
DIM a AS LONG, b AS LONG, m(2) AS LONG, n(2) AS LONG
SWAP (a), b
CALL SWAP(a, b)
SYSTEM
