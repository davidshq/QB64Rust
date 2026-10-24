$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
' Written by tools/opengl/gl_table.py from libqb's gl.h; not edited by hand.
DIM a AS _UNSIGNED LONG, b AS LONG, c AS LONG, d AS LONG, e AS LONG, f AS LONG, g AS _UNSIGNED LONG, h AS _UNSIGNED LONG, i AS _OFFSET
_glTexImage2D a, b, c, d, e, f, g, h, i
END

SUB _GL
END SUB
