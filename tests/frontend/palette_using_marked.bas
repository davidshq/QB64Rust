' TEST: check-fail
' PALETTE USING: the old compiler reads it by hand, outside the template of PALETTE (verification\v27_x64), so no
' form of the table matches it. It is marked "not supported yet", never a syntax error (m3-window-output task 4.4;
' before, the parser reported "matches none of its forms" for a line the old compiler accepts)
DIM a(15) AS INTEGER
PALETTE USING a(0)
PALETTE USING a
SYSTEM
