$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
' Written by tools/opengl/gl_table.py from libqb's gl.h; not edited by hand.
DIM a AS LONG, b AS LONG, c AS LONG, d AS LONG
_glVertex4i a, b, c, d
END

SUB _GL
END SUB
