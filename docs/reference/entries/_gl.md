# SUB _GL

A SUB named `_GL` is the program's OpenGL routine: the runtime calls it once a frame while the window is shown, and
inside it the program calls OpenGL through `_gl…` routines and `_GL_…` constants.

## Syntax

```text
SUB _GL [STATIC]
    ...
END SUB
```

## Behavior

The runtime calls `SUB _GL` once a frame while the program's window is shown, at the layer `_GLRENDER` sets, and the
program is built with the runtime's OpenGL part.
<!-- src: language/opengl "SUB _GL"; verification\v59_a1_plain; verification\v59_h1_behind -->

In a program whose window is never shown (`$SCREENHIDE`, `$CONSOLE:ONLY`) the SUB is never called.
<!-- src: language/opengl "SUB _GL without a shown window"; verification\v59_a5_console_only; verification\v59_a6_screenhide; tests\reference\examples\_gl_1 -->

With `SUB _GL` in the program, each OpenGL constant `GL_…` of QB64pe's `gl.h` is `_GL_…`, an `_INTEGER64`, and each
routine `gl…` is `_gl…`, a SUB or a FUNCTION by its result. `_gluPerspective` is there too. The constants stand
wherever a constant may, but not in a `CONST`.
<!-- src: language/opengl "The OpenGL names"; tests\reference\examples\_gl_1 -->

Every argument of a routine is passed by value, converted to the routine's type as the table below gives it; a pointer
is an `_OFFSET`. `_glGetString` returns a STRING.
<!-- src: language/opengl "The arguments of an OpenGL routine" -->

What a program reads back from OpenGL inside the SUB, where it does not depend on the driver (a cleared color through
`_glReadPixels`, the viewport, `_glIsEnabled`, `_glGetError`), is what QB64pe's program reads.
<!-- src: language/opengl "Reading OpenGL back" -->

`DECLARE SUB _GL ()` is accepted and changes nothing.
<!-- src: language/opengl "SUB _GL"; verification\v59_x34_declare_sub_gl -->

## Errors

- A call of the SUB (`_GL`, `CALL _GL`, also inside itself) is the compile error "Cannot call SUB _GL directly". `SUB
  _GL` with a parameter, `FUNCTION _GL` and a second `SUB _GL` are "Invalid name"; `SUB _GL$` is "Type symbols after
  a SUB name are invalid".
  <!-- src: language/opengl "The forms of SUB _GL the old compiler rejects"; verification\v59_x35_sub_gl_suffix -->
- In a program without `SUB _GL` the OpenGL names do not exist: a statement is "Syntax error", a function or a
  constant "Invalid variable name".
  <!-- src: language/opengl "The OpenGL names where they do not exist" -->
- The wrong number of arguments, or a string where a number goes, is a compile error.
  <!-- src: language/opengl "The arguments of an OpenGL routine"; verification\v59_x36_flush_empty_parens; verification\v59_x38_viewport_omitted -->
- A routine with an argument, or `_glGetString`, called while `SUB _GL` is not running raises error 270, and `END` or
  `SYSTEM` inside the SUB raises error 271. Both end the program, whatever `ON ERROR` says. `_glFlush`,
  `_glGetError` and `_gluPerspective` raise nothing there.
  <!-- src: language/opengl "The errors of OpenGL out of place"; verification\v59_f2_viewport_unhandled; verification\v59_f3_system_unhandled; verification\v59_f6_no_helper -->

## Differences

`STOP` inside `SUB _GL` ends the program with error 271, as `END` and `SYSTEM` do there; QB64pe's program hangs.
<!-- src: language/opengl "STOP inside SUB _GL"; D-036 -->

## The OpenGL routines

<!-- begin generated: opengl -->
337 routines and 585 constants, from `crates/sema/src/gl_table.rs` (QB64pe's copy of `gl.h`). Each constant `_GL_…` is an `_INTEGER64`. Every argument is passed by value; a pointer is an `_OFFSET`.

| Routine | Result | Arguments | Outside `SUB _GL` |
|---|---|---|---|
| `_glAccum` | SUB | _UNSIGNED LONG, SINGLE | error 270 |
| `_glAlphaFunc` | SUB | _UNSIGNED LONG, SINGLE | error 270 |
| `_glAreTexturesResident` | _UNSIGNED _BYTE | LONG, _OFFSET, _OFFSET | error 270 |
| `_glArrayElement` | SUB | LONG | error 270 |
| `_glBegin` | SUB | _UNSIGNED LONG | error 270 |
| `_glBindTexture` | SUB | _UNSIGNED LONG, _UNSIGNED LONG | error 270 |
| `_glBitmap` | SUB | LONG, LONG, SINGLE, SINGLE, SINGLE, SINGLE, _OFFSET | error 270 |
| `_glBlendFunc` | SUB | _UNSIGNED LONG, _UNSIGNED LONG | error 270 |
| `_glCallList` | SUB | _UNSIGNED LONG | error 270 |
| `_glCallLists` | SUB | LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glClear` | SUB | _UNSIGNED LONG | error 270 |
| `_glClearAccum` | SUB | SINGLE, SINGLE, SINGLE, SINGLE | error 270 |
| `_glClearColor` | SUB | SINGLE, SINGLE, SINGLE, SINGLE | error 270 |
| `_glClearDepth` | SUB | DOUBLE | error 270 |
| `_glClearIndex` | SUB | SINGLE | error 270 |
| `_glClearStencil` | SUB | LONG | error 270 |
| `_glClipPlane` | SUB | _UNSIGNED LONG, _OFFSET | error 270 |
| `_glColor3b` | SUB | _BYTE, _BYTE, _BYTE | error 270 |
| `_glColor3bv` | SUB | _OFFSET | error 270 |
| `_glColor3d` | SUB | DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glColor3dv` | SUB | _OFFSET | error 270 |
| `_glColor3f` | SUB | SINGLE, SINGLE, SINGLE | error 270 |
| `_glColor3fv` | SUB | _OFFSET | error 270 |
| `_glColor3i` | SUB | LONG, LONG, LONG | error 270 |
| `_glColor3iv` | SUB | _OFFSET | error 270 |
| `_glColor3s` | SUB | INTEGER, INTEGER, INTEGER | error 270 |
| `_glColor3sv` | SUB | _OFFSET | error 270 |
| `_glColor3ub` | SUB | _UNSIGNED _BYTE, _UNSIGNED _BYTE, _UNSIGNED _BYTE | error 270 |
| `_glColor3ubv` | SUB | _OFFSET | error 270 |
| `_glColor3ui` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _UNSIGNED LONG | error 270 |
| `_glColor3uiv` | SUB | _OFFSET | error 270 |
| `_glColor3us` | SUB | _UNSIGNED INTEGER, _UNSIGNED INTEGER, _UNSIGNED INTEGER | error 270 |
| `_glColor3usv` | SUB | _OFFSET | error 270 |
| `_glColor4b` | SUB | _BYTE, _BYTE, _BYTE, _BYTE | error 270 |
| `_glColor4bv` | SUB | _OFFSET | error 270 |
| `_glColor4d` | SUB | DOUBLE, DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glColor4dv` | SUB | _OFFSET | error 270 |
| `_glColor4f` | SUB | SINGLE, SINGLE, SINGLE, SINGLE | error 270 |
| `_glColor4fv` | SUB | _OFFSET | error 270 |
| `_glColor4i` | SUB | LONG, LONG, LONG, LONG | error 270 |
| `_glColor4iv` | SUB | _OFFSET | error 270 |
| `_glColor4s` | SUB | INTEGER, INTEGER, INTEGER, INTEGER | error 270 |
| `_glColor4sv` | SUB | _OFFSET | error 270 |
| `_glColor4ub` | SUB | _UNSIGNED _BYTE, _UNSIGNED _BYTE, _UNSIGNED _BYTE, _UNSIGNED _BYTE | error 270 |
| `_glColor4ubv` | SUB | _OFFSET | error 270 |
| `_glColor4ui` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _UNSIGNED LONG, _UNSIGNED LONG | error 270 |
| `_glColor4uiv` | SUB | _OFFSET | error 270 |
| `_glColor4us` | SUB | _UNSIGNED INTEGER, _UNSIGNED INTEGER, _UNSIGNED INTEGER, _UNSIGNED INTEGER | error 270 |
| `_glColor4usv` | SUB | _OFFSET | error 270 |
| `_glColorMask` | SUB | _UNSIGNED _BYTE, _UNSIGNED _BYTE, _UNSIGNED _BYTE, _UNSIGNED _BYTE | error 270 |
| `_glColorMaterial` | SUB | _UNSIGNED LONG, _UNSIGNED LONG | error 270 |
| `_glColorPointer` | SUB | LONG, _UNSIGNED LONG, LONG, _OFFSET | error 270 |
| `_glCopyPixels` | SUB | LONG, LONG, LONG, LONG, _UNSIGNED LONG | error 270 |
| `_glCopyTexImage1D` | SUB | _UNSIGNED LONG, LONG, _UNSIGNED LONG, LONG, LONG, LONG, LONG | error 270 |
| `_glCopyTexImage2D` | SUB | _UNSIGNED LONG, LONG, _UNSIGNED LONG, LONG, LONG, LONG, LONG, LONG | error 270 |
| `_glCopyTexSubImage1D` | SUB | _UNSIGNED LONG, LONG, LONG, LONG, LONG, LONG | error 270 |
| `_glCopyTexSubImage2D` | SUB | _UNSIGNED LONG, LONG, LONG, LONG, LONG, LONG, LONG, LONG | error 270 |
| `_glCullFace` | SUB | _UNSIGNED LONG | error 270 |
| `_glDeleteLists` | SUB | _UNSIGNED LONG, LONG | error 270 |
| `_glDeleteTextures` | SUB | LONG, _OFFSET | error 270 |
| `_glDepthFunc` | SUB | _UNSIGNED LONG | error 270 |
| `_glDepthMask` | SUB | _UNSIGNED _BYTE | error 270 |
| `_glDepthRange` | SUB | DOUBLE, DOUBLE | error 270 |
| `_glDisable` | SUB | _UNSIGNED LONG | error 270 |
| `_glDisableClientState` | SUB | _UNSIGNED LONG | error 270 |
| `_glDrawArrays` | SUB | _UNSIGNED LONG, LONG, LONG | error 270 |
| `_glDrawBuffer` | SUB | _UNSIGNED LONG | error 270 |
| `_glDrawElements` | SUB | _UNSIGNED LONG, LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glDrawPixels` | SUB | LONG, LONG, _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glEdgeFlag` | SUB | _UNSIGNED _BYTE | error 270 |
| `_glEdgeFlagPointer` | SUB | LONG, _OFFSET | error 270 |
| `_glEdgeFlagv` | SUB | _OFFSET | error 270 |
| `_glEnable` | SUB | _UNSIGNED LONG | error 270 |
| `_glEnableClientState` | SUB | _UNSIGNED LONG | error 270 |
| `_glEnd` | SUB | none | no error |
| `_glEndList` | SUB | none | no error |
| `_glEvalCoord1d` | SUB | DOUBLE | error 270 |
| `_glEvalCoord1dv` | SUB | _OFFSET | error 270 |
| `_glEvalCoord1f` | SUB | SINGLE | error 270 |
| `_glEvalCoord1fv` | SUB | _OFFSET | error 270 |
| `_glEvalCoord2d` | SUB | DOUBLE, DOUBLE | error 270 |
| `_glEvalCoord2dv` | SUB | _OFFSET | error 270 |
| `_glEvalCoord2f` | SUB | SINGLE, SINGLE | error 270 |
| `_glEvalCoord2fv` | SUB | _OFFSET | error 270 |
| `_glEvalMesh1` | SUB | _UNSIGNED LONG, LONG, LONG | error 270 |
| `_glEvalMesh2` | SUB | _UNSIGNED LONG, LONG, LONG, LONG, LONG | error 270 |
| `_glEvalPoint1` | SUB | LONG | error 270 |
| `_glEvalPoint2` | SUB | LONG, LONG | error 270 |
| `_glFeedbackBuffer` | SUB | LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glFinish` | SUB | none | no error |
| `_glFlush` | SUB | none | no error |
| `_glFogf` | SUB | _UNSIGNED LONG, SINGLE | error 270 |
| `_glFogfv` | SUB | _UNSIGNED LONG, _OFFSET | error 270 |
| `_glFogi` | SUB | _UNSIGNED LONG, LONG | error 270 |
| `_glFogiv` | SUB | _UNSIGNED LONG, _OFFSET | error 270 |
| `_glFrontFace` | SUB | _UNSIGNED LONG | error 270 |
| `_glFrustum` | SUB | DOUBLE, DOUBLE, DOUBLE, DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glGenLists` | _UNSIGNED LONG | LONG | error 270 |
| `_glGenTextures` | SUB | LONG, _OFFSET | error 270 |
| `_glGetBooleanv` | SUB | _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetClipPlane` | SUB | _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetDoublev` | SUB | _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetError` | _UNSIGNED LONG | none | no error |
| `_glGetFloatv` | SUB | _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetIntegerv` | SUB | _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetLightfv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetLightiv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetMapdv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetMapfv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetMapiv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetMaterialfv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetMaterialiv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetPixelMapfv` | SUB | _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetPixelMapuiv` | SUB | _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetPixelMapusv` | SUB | _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetPointerv` | SUB | _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetPolygonStipple` | SUB | _OFFSET | error 270 |
| `_glGetString` | STRING | _UNSIGNED LONG | error 270 |
| `_glGetTexEnvfv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetTexEnviv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetTexGendv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetTexGenfv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetTexGeniv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetTexImage` | SUB | _UNSIGNED LONG, LONG, _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetTexLevelParameterfv` | SUB | _UNSIGNED LONG, LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetTexLevelParameteriv` | SUB | _UNSIGNED LONG, LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetTexParameterfv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glGetTexParameteriv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glHint` | SUB | _UNSIGNED LONG, _UNSIGNED LONG | error 270 |
| `_glIndexd` | SUB | DOUBLE | error 270 |
| `_glIndexdv` | SUB | _OFFSET | error 270 |
| `_glIndexf` | SUB | SINGLE | error 270 |
| `_glIndexfv` | SUB | _OFFSET | error 270 |
| `_glIndexi` | SUB | LONG | error 270 |
| `_glIndexiv` | SUB | _OFFSET | error 270 |
| `_glIndexMask` | SUB | _UNSIGNED LONG | error 270 |
| `_glIndexPointer` | SUB | _UNSIGNED LONG, LONG, _OFFSET | error 270 |
| `_glIndexs` | SUB | INTEGER | error 270 |
| `_glIndexsv` | SUB | _OFFSET | error 270 |
| `_glIndexub` | SUB | _UNSIGNED _BYTE | error 270 |
| `_glIndexubv` | SUB | _OFFSET | error 270 |
| `_glInitNames` | SUB | none | no error |
| `_glInterleavedArrays` | SUB | _UNSIGNED LONG, LONG, _OFFSET | error 270 |
| `_glIsEnabled` | _UNSIGNED _BYTE | _UNSIGNED LONG | error 270 |
| `_glIsList` | _UNSIGNED _BYTE | _UNSIGNED LONG | error 270 |
| `_glIsTexture` | _UNSIGNED _BYTE | _UNSIGNED LONG | error 270 |
| `_glLightf` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, SINGLE | error 270 |
| `_glLightfv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glLighti` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, LONG | error 270 |
| `_glLightiv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glLightModelf` | SUB | _UNSIGNED LONG, SINGLE | error 270 |
| `_glLightModelfv` | SUB | _UNSIGNED LONG, _OFFSET | error 270 |
| `_glLightModeli` | SUB | _UNSIGNED LONG, LONG | error 270 |
| `_glLightModeliv` | SUB | _UNSIGNED LONG, _OFFSET | error 270 |
| `_glLineStipple` | SUB | LONG, _UNSIGNED INTEGER | error 270 |
| `_glLineWidth` | SUB | SINGLE | error 270 |
| `_glListBase` | SUB | _UNSIGNED LONG | error 270 |
| `_glLoadIdentity` | SUB | none | no error |
| `_glLoadMatrixd` | SUB | _OFFSET | error 270 |
| `_glLoadMatrixf` | SUB | _OFFSET | error 270 |
| `_glLoadName` | SUB | _UNSIGNED LONG | error 270 |
| `_glLogicOp` | SUB | _UNSIGNED LONG | error 270 |
| `_glMap1d` | SUB | _UNSIGNED LONG, DOUBLE, DOUBLE, LONG, LONG, _OFFSET | error 270 |
| `_glMap1f` | SUB | _UNSIGNED LONG, SINGLE, SINGLE, LONG, LONG, _OFFSET | error 270 |
| `_glMap2d` | SUB | _UNSIGNED LONG, DOUBLE, DOUBLE, LONG, LONG, DOUBLE, DOUBLE, LONG, LONG, _OFFSET | error 270 |
| `_glMap2f` | SUB | _UNSIGNED LONG, SINGLE, SINGLE, LONG, LONG, SINGLE, SINGLE, LONG, LONG, _OFFSET | error 270 |
| `_glMapGrid1d` | SUB | LONG, DOUBLE, DOUBLE | error 270 |
| `_glMapGrid1f` | SUB | LONG, SINGLE, SINGLE | error 270 |
| `_glMapGrid2d` | SUB | LONG, DOUBLE, DOUBLE, LONG, DOUBLE, DOUBLE | error 270 |
| `_glMapGrid2f` | SUB | LONG, SINGLE, SINGLE, LONG, SINGLE, SINGLE | error 270 |
| `_glMaterialf` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, SINGLE | error 270 |
| `_glMaterialfv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glMateriali` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, LONG | error 270 |
| `_glMaterialiv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glMatrixMode` | SUB | _UNSIGNED LONG | error 270 |
| `_glMultMatrixd` | SUB | _OFFSET | error 270 |
| `_glMultMatrixf` | SUB | _OFFSET | error 270 |
| `_glNewList` | SUB | _UNSIGNED LONG, _UNSIGNED LONG | error 270 |
| `_glNormal3b` | SUB | _BYTE, _BYTE, _BYTE | error 270 |
| `_glNormal3bv` | SUB | _OFFSET | error 270 |
| `_glNormal3d` | SUB | DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glNormal3dv` | SUB | _OFFSET | error 270 |
| `_glNormal3f` | SUB | SINGLE, SINGLE, SINGLE | error 270 |
| `_glNormal3fv` | SUB | _OFFSET | error 270 |
| `_glNormal3i` | SUB | LONG, LONG, LONG | error 270 |
| `_glNormal3iv` | SUB | _OFFSET | error 270 |
| `_glNormal3s` | SUB | INTEGER, INTEGER, INTEGER | error 270 |
| `_glNormal3sv` | SUB | _OFFSET | error 270 |
| `_glNormalPointer` | SUB | _UNSIGNED LONG, LONG, _OFFSET | error 270 |
| `_glOrtho` | SUB | DOUBLE, DOUBLE, DOUBLE, DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glPassThrough` | SUB | SINGLE | error 270 |
| `_glPixelMapfv` | SUB | _UNSIGNED LONG, LONG, _OFFSET | error 270 |
| `_glPixelMapuiv` | SUB | _UNSIGNED LONG, LONG, _OFFSET | error 270 |
| `_glPixelMapusv` | SUB | _UNSIGNED LONG, LONG, _OFFSET | error 270 |
| `_glPixelStoref` | SUB | _UNSIGNED LONG, SINGLE | error 270 |
| `_glPixelStorei` | SUB | _UNSIGNED LONG, LONG | error 270 |
| `_glPixelTransferf` | SUB | _UNSIGNED LONG, SINGLE | error 270 |
| `_glPixelTransferi` | SUB | _UNSIGNED LONG, LONG | error 270 |
| `_glPixelZoom` | SUB | SINGLE, SINGLE | error 270 |
| `_glPointSize` | SUB | SINGLE | error 270 |
| `_glPolygonMode` | SUB | _UNSIGNED LONG, _UNSIGNED LONG | error 270 |
| `_glPolygonOffset` | SUB | SINGLE, SINGLE | error 270 |
| `_glPolygonStipple` | SUB | _OFFSET | error 270 |
| `_glPopAttrib` | SUB | none | no error |
| `_glPopClientAttrib` | SUB | none | no error |
| `_glPopMatrix` | SUB | none | no error |
| `_glPopName` | SUB | none | no error |
| `_glPrioritizeTextures` | SUB | LONG, _OFFSET, _OFFSET | error 270 |
| `_glPushAttrib` | SUB | _UNSIGNED LONG | error 270 |
| `_glPushClientAttrib` | SUB | _UNSIGNED LONG | error 270 |
| `_glPushMatrix` | SUB | none | no error |
| `_glPushName` | SUB | _UNSIGNED LONG | error 270 |
| `_glRasterPos2d` | SUB | DOUBLE, DOUBLE | error 270 |
| `_glRasterPos2dv` | SUB | _OFFSET | error 270 |
| `_glRasterPos2f` | SUB | SINGLE, SINGLE | error 270 |
| `_glRasterPos2fv` | SUB | _OFFSET | error 270 |
| `_glRasterPos2i` | SUB | LONG, LONG | error 270 |
| `_glRasterPos2iv` | SUB | _OFFSET | error 270 |
| `_glRasterPos2s` | SUB | INTEGER, INTEGER | error 270 |
| `_glRasterPos2sv` | SUB | _OFFSET | error 270 |
| `_glRasterPos3d` | SUB | DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glRasterPos3dv` | SUB | _OFFSET | error 270 |
| `_glRasterPos3f` | SUB | SINGLE, SINGLE, SINGLE | error 270 |
| `_glRasterPos3fv` | SUB | _OFFSET | error 270 |
| `_glRasterPos3i` | SUB | LONG, LONG, LONG | error 270 |
| `_glRasterPos3iv` | SUB | _OFFSET | error 270 |
| `_glRasterPos3s` | SUB | INTEGER, INTEGER, INTEGER | error 270 |
| `_glRasterPos3sv` | SUB | _OFFSET | error 270 |
| `_glRasterPos4d` | SUB | DOUBLE, DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glRasterPos4dv` | SUB | _OFFSET | error 270 |
| `_glRasterPos4f` | SUB | SINGLE, SINGLE, SINGLE, SINGLE | error 270 |
| `_glRasterPos4fv` | SUB | _OFFSET | error 270 |
| `_glRasterPos4i` | SUB | LONG, LONG, LONG, LONG | error 270 |
| `_glRasterPos4iv` | SUB | _OFFSET | error 270 |
| `_glRasterPos4s` | SUB | INTEGER, INTEGER, INTEGER, INTEGER | error 270 |
| `_glRasterPos4sv` | SUB | _OFFSET | error 270 |
| `_glReadBuffer` | SUB | _UNSIGNED LONG | error 270 |
| `_glReadPixels` | SUB | LONG, LONG, LONG, LONG, _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glRectd` | SUB | DOUBLE, DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glRectdv` | SUB | _OFFSET, _OFFSET | error 270 |
| `_glRectf` | SUB | SINGLE, SINGLE, SINGLE, SINGLE | error 270 |
| `_glRectfv` | SUB | _OFFSET, _OFFSET | error 270 |
| `_glRecti` | SUB | LONG, LONG, LONG, LONG | error 270 |
| `_glRectiv` | SUB | _OFFSET, _OFFSET | error 270 |
| `_glRects` | SUB | INTEGER, INTEGER, INTEGER, INTEGER | error 270 |
| `_glRectsv` | SUB | _OFFSET, _OFFSET | error 270 |
| `_glRenderMode` | LONG | _UNSIGNED LONG | error 270 |
| `_glRotated` | SUB | DOUBLE, DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glRotatef` | SUB | SINGLE, SINGLE, SINGLE, SINGLE | error 270 |
| `_glScaled` | SUB | DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glScalef` | SUB | SINGLE, SINGLE, SINGLE | error 270 |
| `_glScissor` | SUB | LONG, LONG, LONG, LONG | error 270 |
| `_glSelectBuffer` | SUB | LONG, _OFFSET | error 270 |
| `_glShadeModel` | SUB | _UNSIGNED LONG | error 270 |
| `_glStencilFunc` | SUB | _UNSIGNED LONG, LONG, _UNSIGNED LONG | error 270 |
| `_glStencilMask` | SUB | _UNSIGNED LONG | error 270 |
| `_glStencilOp` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _UNSIGNED LONG | error 270 |
| `_glTexCoord1d` | SUB | DOUBLE | error 270 |
| `_glTexCoord1dv` | SUB | _OFFSET | error 270 |
| `_glTexCoord1f` | SUB | SINGLE | error 270 |
| `_glTexCoord1fv` | SUB | _OFFSET | error 270 |
| `_glTexCoord1i` | SUB | LONG | error 270 |
| `_glTexCoord1iv` | SUB | _OFFSET | error 270 |
| `_glTexCoord1s` | SUB | INTEGER | error 270 |
| `_glTexCoord1sv` | SUB | _OFFSET | error 270 |
| `_glTexCoord2d` | SUB | DOUBLE, DOUBLE | error 270 |
| `_glTexCoord2dv` | SUB | _OFFSET | error 270 |
| `_glTexCoord2f` | SUB | SINGLE, SINGLE | error 270 |
| `_glTexCoord2fv` | SUB | _OFFSET | error 270 |
| `_glTexCoord2i` | SUB | LONG, LONG | error 270 |
| `_glTexCoord2iv` | SUB | _OFFSET | error 270 |
| `_glTexCoord2s` | SUB | INTEGER, INTEGER | error 270 |
| `_glTexCoord2sv` | SUB | _OFFSET | error 270 |
| `_glTexCoord3d` | SUB | DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glTexCoord3dv` | SUB | _OFFSET | error 270 |
| `_glTexCoord3f` | SUB | SINGLE, SINGLE, SINGLE | error 270 |
| `_glTexCoord3fv` | SUB | _OFFSET | error 270 |
| `_glTexCoord3i` | SUB | LONG, LONG, LONG | error 270 |
| `_glTexCoord3iv` | SUB | _OFFSET | error 270 |
| `_glTexCoord3s` | SUB | INTEGER, INTEGER, INTEGER | error 270 |
| `_glTexCoord3sv` | SUB | _OFFSET | error 270 |
| `_glTexCoord4d` | SUB | DOUBLE, DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glTexCoord4dv` | SUB | _OFFSET | error 270 |
| `_glTexCoord4f` | SUB | SINGLE, SINGLE, SINGLE, SINGLE | error 270 |
| `_glTexCoord4fv` | SUB | _OFFSET | error 270 |
| `_glTexCoord4i` | SUB | LONG, LONG, LONG, LONG | error 270 |
| `_glTexCoord4iv` | SUB | _OFFSET | error 270 |
| `_glTexCoord4s` | SUB | INTEGER, INTEGER, INTEGER, INTEGER | error 270 |
| `_glTexCoord4sv` | SUB | _OFFSET | error 270 |
| `_glTexCoordPointer` | SUB | LONG, _UNSIGNED LONG, LONG, _OFFSET | error 270 |
| `_glTexEnvf` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, SINGLE | error 270 |
| `_glTexEnvfv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glTexEnvi` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, LONG | error 270 |
| `_glTexEnviv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glTexGend` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, DOUBLE | error 270 |
| `_glTexGendv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glTexGenf` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, SINGLE | error 270 |
| `_glTexGenfv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glTexGeni` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, LONG | error 270 |
| `_glTexGeniv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glTexImage1D` | SUB | _UNSIGNED LONG, LONG, LONG, LONG, LONG, _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glTexImage2D` | SUB | _UNSIGNED LONG, LONG, LONG, LONG, LONG, LONG, _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glTexParameterf` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, SINGLE | error 270 |
| `_glTexParameterfv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glTexParameteri` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, LONG | error 270 |
| `_glTexParameteriv` | SUB | _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glTexSubImage1D` | SUB | _UNSIGNED LONG, LONG, LONG, LONG, _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glTexSubImage2D` | SUB | _UNSIGNED LONG, LONG, LONG, LONG, LONG, LONG, _UNSIGNED LONG, _UNSIGNED LONG, _OFFSET | error 270 |
| `_glTranslated` | SUB | DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glTranslatef` | SUB | SINGLE, SINGLE, SINGLE | error 270 |
| `_gluPerspective` | SUB | DOUBLE, DOUBLE, DOUBLE, DOUBLE | no error |
| `_glVertex2d` | SUB | DOUBLE, DOUBLE | error 270 |
| `_glVertex2dv` | SUB | _OFFSET | error 270 |
| `_glVertex2f` | SUB | SINGLE, SINGLE | error 270 |
| `_glVertex2fv` | SUB | _OFFSET | error 270 |
| `_glVertex2i` | SUB | LONG, LONG | error 270 |
| `_glVertex2iv` | SUB | _OFFSET | error 270 |
| `_glVertex2s` | SUB | INTEGER, INTEGER | error 270 |
| `_glVertex2sv` | SUB | _OFFSET | error 270 |
| `_glVertex3d` | SUB | DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glVertex3dv` | SUB | _OFFSET | error 270 |
| `_glVertex3f` | SUB | SINGLE, SINGLE, SINGLE | error 270 |
| `_glVertex3fv` | SUB | _OFFSET | error 270 |
| `_glVertex3i` | SUB | LONG, LONG, LONG | error 270 |
| `_glVertex3iv` | SUB | _OFFSET | error 270 |
| `_glVertex3s` | SUB | INTEGER, INTEGER, INTEGER | error 270 |
| `_glVertex3sv` | SUB | _OFFSET | error 270 |
| `_glVertex4d` | SUB | DOUBLE, DOUBLE, DOUBLE, DOUBLE | error 270 |
| `_glVertex4dv` | SUB | _OFFSET | error 270 |
| `_glVertex4f` | SUB | SINGLE, SINGLE, SINGLE, SINGLE | error 270 |
| `_glVertex4fv` | SUB | _OFFSET | error 270 |
| `_glVertex4i` | SUB | LONG, LONG, LONG, LONG | error 270 |
| `_glVertex4iv` | SUB | _OFFSET | error 270 |
| `_glVertex4s` | SUB | INTEGER, INTEGER, INTEGER, INTEGER | error 270 |
| `_glVertex4sv` | SUB | _OFFSET | error 270 |
| `_glVertexPointer` | SUB | LONG, _UNSIGNED LONG, LONG, _OFFSET | error 270 |
| `_glViewport` | SUB | LONG, LONG, LONG, LONG | error 270 |
<!-- end generated -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:_gl_1 -->
```qb64
' A console program shows no window, so the runtime never calls SUB _GL.
DIM SHARED frames AS LONG
_DELAY 0.2
PRINT "Frames drawn in a console program:"; frames
PRINT "_GL_COLOR_BUFFER_BIT is"; _GL_COLOR_BUFFER_BIT
END

SUB _GL
    frames = frames + 1
END SUB
```

Output:

```text
Frames drawn in a console program: 0 
_GL_COLOR_BUFFER_BIT is 16384 

Press any key to continue
```
<!-- end generated -->

## See also

`_GLRENDER`, `_DISPLAYORDER`, `_DEPTHBUFFER`, `SUB`.
