# OpenGL

A program draws with OpenGL inside a SUB named `_GL`, which the runtime calls once a frame. Inside it the program has
QB64pe's OpenGL routines as `_gl…` and its constants as `_GL_…`.

The pages of this area: [SUB _GL](../entries/_gl.md), [_GLRENDER](../entries/_glrender.md),
[_DISPLAYORDER](../entries/_displayorder.md), [_DEPTHBUFFER](../entries/_depthbuffer.md).

## SUB _GL

The runtime calls the SUB once a frame while the window is shown, at the layer `_GLRENDER` sets. A program never calls
it itself, and a program whose window is never shown never sees it called.
<!-- src: language/opengl "SUB _GL"; language/opengl "SUB _GL without a shown window"; tests\reference\examples\_gl_1 -->

The routines and constants exist only in a program with `SUB _GL`. Every argument is passed by value, converted to the
routine's type; the page of `SUB _GL` lists all 337 routines.
<!-- src: language/opengl "The OpenGL names"; language/opengl "The arguments of an OpenGL routine" -->

## Out of place

Most routines called while the SUB is not running raise error 270, and `END`, `SYSTEM` or `STOP` inside it error 271.
Both end the program whatever `ON ERROR` says.
<!-- src: language/opengl "The errors of OpenGL out of place"; language/opengl "STOP inside SUB _GL" -->
