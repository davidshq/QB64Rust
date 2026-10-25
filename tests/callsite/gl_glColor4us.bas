$CONSOLE:ONLY
' Call-site check (tools/callsite): the statement before END is compared with the old compiler's C++.
' Written by tools/opengl/gl_table.py from libqb's gl.h; not edited by hand.
DIM a AS _UNSIGNED INTEGER, b AS _UNSIGNED INTEGER, c AS _UNSIGNED INTEGER, d AS _UNSIGNED INTEGER
_glColor4us a, b, c, d
END

SUB _GL
END SUB
