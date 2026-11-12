$CONSOLE:ONLY
' A console program shows no window, so the runtime never calls SUB _GL.
DIM SHARED frames AS LONG
_DELAY 0.2
PRINT "Frames drawn in a console program:"; frames
PRINT "_GL_COLOR_BUFFER_BIT is"; _GL_COLOR_BUFFER_BIT
END

SUB _GL
    frames = frames + 1
END SUB
