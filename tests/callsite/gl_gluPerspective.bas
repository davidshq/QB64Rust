$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
' Written by tools/opengl/gl_table.py from libqb's gl.h; not edited by hand.
DIM a AS DOUBLE, b AS DOUBLE, c AS DOUBLE, d AS DOUBLE
_gluPerspective a, b, c, d
END

SUB _GL
END SUB
