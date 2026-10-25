$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
' Written by tools/opengl/gl_table.py from libqb's gl.h; not edited by hand.
DIM a AS LONG, b AS _UNSIGNED LONG, c AS _OFFSET
_glCallLists a, b, c
END

SUB _GL
END SUB
