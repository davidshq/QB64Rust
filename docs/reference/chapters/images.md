# Images

An image is a surface of pixels with a handle. A program makes, loads, copies and frees images, draws on them by
making one the destination, and copies between them with `_PUTIMAGE`.

The pages of this area: [_NEWIMAGE](../entries/_newimage.md), [_LOADIMAGE](../entries/_loadimage.md),
[_SAVEIMAGE](../entries/_saveimage.md), [_COPYIMAGE](../entries/_copyimage.md),
[_FREEIMAGE](../entries/_freeimage.md), [_PUTIMAGE](../entries/_putimage.md), [_DEST](../entries/_dest.md),
[_SOURCE](../entries/_source.md), [_WIDTH](../entries/_width.md), [_PIXELSIZE](../entries/_pixelsize.md),
[_CLEARCOLOR](../entries/_clearcolor.md), [_SETALPHA](../entries/_setalpha.md), [_BLEND](../entries/_blend.md),
[_RGB32](../entries/_rgb32.md), [_RGB](../entries/_rgb.md), [_RED32](../entries/_red32.md),
[_MAPTRIANGLE](../entries/_maptriangle.md).

## Handles

`_NEWIMAGE(w, h, mode)` gives a handle below -1 of a new image: mode 32 for 32-bit color, 256 for a palette, 0 for
text. `_DEST` and `_SOURCE` choose the image that statements draw on and read from.
<!-- src: language/images "Image handles"; tests\reference\examples\_newimage_1 -->

A handle of no image raises a run-time error; after `_FREEIMAGE` the handle names nothing.
<!-- src: language/images "Image handles"; tests\reference\examples\_freeimage_1 -->

## Between images

`_PUTIMAGE` copies a rectangle of one image onto another, scaled to the rectangle given. `_CLEARCOLOR` and `_SETALPHA`
make colors transparent, and `_BLEND` decides whether alpha blends.
<!-- src: language/images "Drawing between images"; language/images "Transparency, blending and palettes"; tests\reference\examples\_putimage_1 -->

`_LOADIMAGE` and `_SAVEIMAGE` read and write image files.
<!-- src: language/images "Loading and saving images" -->
