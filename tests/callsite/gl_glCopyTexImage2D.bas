$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
' Written by tools/opengl/gl_table.py from libqb's gl.h; not edited by hand.
DIM a AS _UNSIGNED LONG, b AS LONG, c AS _UNSIGNED LONG, d AS LONG, e AS LONG, f AS LONG, g AS LONG, h AS LONG
_glCopyTexImage2D a, b, c, d, e, f, g, h
END

SUB _GL
END SUB
