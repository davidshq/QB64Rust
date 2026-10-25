' TEST: ir
$CONSOLE:ONLY
' FOR over an element of a member array (m4-left-marked task 5.1, design D4; verification\v56_f3, v60_c1-c6): the
' loop names the element's place with its index three times, as the old compiler: the load and the store at NEXT,
' then the test, and the store at the entry from the header, so idx runs once at the start and twice at each NEXT
' (7 times for four entries). A pending error is serviced at the FOR line, and RESUME NEXT goes on into the body.
TYPE t
    m(3) AS INTEGER
END TYPE
DIM SHARED c AS INTEGER
DIM r AS t
FOR r.m(idx(3)) = 1 TO 3: PRINT r.m(3);: NEXT
SYSTEM

FUNCTION idx (k AS INTEGER)
    c = c + 1
    idx = k
END FUNCTION
