$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
DIM m AS _MEM, o AS _OFFSET, l AS LONG, d AS DOUBLE, s5 AS STRING * 5, s AS STRING, x AS LONG
$CHECKING:OFF
x = _MEMGET(m, o, LONG)
END
