# language/images Specification

## Purpose
Image handles and what a program draws between them: loading, copying, saving and freeing images, `_PUTIMAGE`
and `_MAPTRIANGLE`, an image's size, depth, palette, transparency and blending, `_AUTODISPLAY`, `PMAP`, and the
functions that build a colour and take one apart, as the old compiler's program behaves.

## Requirements

### Requirement: Forms of the image built-ins
Every image and colour built-in SHALL compile in each form the old compiler accepts, with the runtime call the old
compiler writes for it: the arguments it allows to be left out, the words of its template, each argument
converted to its slot as the old compiler converts it. A form the old compiler rejects SHALL be a compile error
that is not "not supported yet".

#### Scenario: _PUTIMAGE with parts of its template left out
- **WHEN** `_PUTIMAGE`, `_PUTIMAGE (10, 10), img`, `_PUTIMAGE (0, 0)-(99, 99), img, 0`, `_PUTIMAGE STEP(5, 5),
  img`, `_PUTIMAGE , img, 0, (0, 0)-(9, 9)` and `_PUTIMAGE (0, 0)-(9, 9), img, , , _SMOOTH` are compiled
- **THEN** each compiles to the call the old compiler writes for it

#### Scenario: Arguments of any numeric type
- **WHEN** `_PUTIMAGE (x!, y#), h&, d%` and `PRINT _WIDTH(h&&); _RED(c~&, h%)` are compiled
- **THEN** each argument is converted as the old compiler converts it

#### Scenario: A rejected form
- **WHEN** a form the old compiler rejects, such as `_PUTIMAGE (1, 2)-` or `_LOADIMAGE()`, is compiled
- **THEN** it is a compile error and not "not supported yet"

### Requirement: Result types of the image and colour functions
Each image and colour function SHALL have the result type the old compiler gives it, as `PRINT`, `HEX$` and an
operation on its value show it.

#### Scenario: A handle and a colour
- **WHEN** `PRINT HEX$(_LOADIMAGE("none.bmp", 32)); HEX$(_RGB(255, 0, 0)); HEX$(_RED32(&HFF102030))` runs
- **THEN** it prints what the old compiler's program prints

### Requirement: Image handles
`_NEWIMAGE`, `_LOADIMAGE` and `_COPYIMAGE` SHALL give the handle the old compiler's program gives, `_FREEIMAGE`
SHALL free it, and `_WIDTH`, `_HEIGHT` and `_PIXELSIZE` SHALL give an image's size and depth, of the destination
when no handle is given. A handle that is not valid SHALL raise the error the old compiler's program raises.

#### Scenario: A file that is not there
- **WHEN** `l = _LOADIMAGE("./foobar.bmp", 32): PRINT l` runs in a folder with no such file
- **THEN** it prints `-1`, as the old compiler's program does

#### Scenario: Size and depth
- **WHEN** a program makes `_NEWIMAGE(64, 48, 256)`, copies it with `_COPYIMAGE` and prints `_WIDTH`, `_HEIGHT`
  and `_PIXELSIZE` of both and of the screen
- **THEN** it prints what the old compiler's program prints

#### Scenario: A freed handle
- **WHEN** a program frees an image and then asks its `_WIDTH` under an error handler
- **THEN** the handler sees the error number the old compiler's program sees

### Requirement: Loading and saving images
`_LOADIMAGE` SHALL load an image file as libqb does, into the mode its second argument names or the screen's,
and `_SAVEIMAGE` SHALL write an image to a file as libqb does, so that an image saved and loaded again holds the
pixels the old compiler's program finds.

#### Scenario: An image from a data file
- **WHEN** a screen program loads a BMP and a PNG file from its data files and puts them on the screen
- **THEN** the state of its screen equals the old compiler's

#### Scenario: Saved and loaded again
- **WHEN** a program draws on an image, saves it with `_SAVEIMAGE`, loads the file, and prints `POINT` of some
  pixels of the loaded image
- **THEN** it prints what the old compiler's program prints

### Requirement: Drawing between images
`_PUTIMAGE` SHALL copy a rectangle of one image to a rectangle of another, scaled, mirrored or clipped as libqb
does, `_SOURCE` and `_DEST` choosing the images where the statement leaves them out, and the 2-D form of
`_MAPTRIANGLE` SHALL map a triangle of one image to a triangle of another, so that the screen shows what the old
compiler's program shows.

#### Scenario: Scaled and mirrored
- **WHEN** a screen program puts an image on the screen once at its size, once scaled to twice its size and once
  with its corners swapped
- **THEN** the state of its screen equals the old compiler's

#### Scenario: A triangle mapped
- **WHEN** a screen program maps two triangles of an image to the screen with `_MAPTRIANGLE`
- **THEN** the state of its screen equals the old compiler's

### Requirement: Transparency, blending and palettes
`_CLEARCOLOR`, `_SETALPHA`, `_BLEND`, `_DONTBLEND`, `_PALETTECOLOR` and `_COPYPALETTE` SHALL change an image as
libqb does, and their functions SHALL give what the old compiler's program gives.

#### Scenario: A clear colour
- **WHEN** a screen program sets a clear colour on an image and puts it over a drawing
- **THEN** the state of its screen equals the old compiler's, and `_CLEARCOLOR(img)` prints what it prints there

#### Scenario: Blending in 32 bits
- **WHEN** a screen program in a 32-bit mode draws with half-transparent colours with `_BLEND` and with
  `_DONTBLEND`
- **THEN** the state of its screen equals the old compiler's

#### Scenario: A palette entry
- **WHEN** a program sets `_PALETTECOLOR 3, _RGB32(10, 20, 30)` in a 256-colour mode and prints
  `_PALETTECOLOR(3)`
- **THEN** it prints what the old compiler's program prints

### Requirement: The colour functions
`_RGB`, `_RGBA`, `_RED`, `_GREEN`, `_BLUE` and `_ALPHA` SHALL build and take apart a colour of an image (the
nearest palette entry in a palette mode), and `_RED32`, `_GREEN32`, `_BLUE32`, `_ALPHA32`, `_HSB32`, `_HSBA32`,
`_HUE32`, `_SATURATION32` and `_BRIGHTNESS32` a 32-bit colour, each giving what the old compiler's program gives.

#### Scenario: A colour in a palette mode
- **WHEN** `SCREEN 13: PRINT _RGB(255, 0, 0); _RED(4); _GREEN(4)` runs
- **THEN** it prints what the old compiler's program prints

#### Scenario: Hue, saturation and brightness
- **WHEN** a program prints `_HSB32(120, 50, 50)` and the hue, saturation and brightness of `_RGB32(10, 200, 30)`
- **THEN** it prints what the old compiler's program prints

### Requirement: _AUTODISPLAY and PMAP
`_AUTODISPLAY` SHALL undo `_DISPLAY`'s manual display as libqb does, `_AUTODISPLAY` as a function SHALL give
whether it is on, and `PMAP` SHALL convert between the coordinates of `WINDOW` and of the viewport as the old
compiler's program does.

#### Scenario: PMAP after WINDOW
- **WHEN** `SCREEN 12: WINDOW (0, 0)-(100, 100): PRINT PMAP(50, 0); PMAP(50, 1); PMAP(320, 2)` runs
- **THEN** it prints what the old compiler's program prints

### Requirement: Image built-ins that no test runs
`_SCREENIMAGE`, `_CLIPBOARDIMAGE` (function and statement), `_PRINTIMAGE`, `_DEPTHBUFFER` and the 3-D form of
`_MAPTRIANGLE` SHALL compile to the call the old compiler writes, and SHALL link the part of libqb each needs; no
test runs them, since what they read or draw is the desktop's, the clipboard's, a printer's or OpenGL's.

#### Scenario: A desktop image
- **WHEN** `i = _SCREENIMAGE` is compiled
- **THEN** it compiles to the call the old compiler writes, and the program is linked with the screen-capture
  part of libqb

#### Scenario: A desktop image takes no corners or all four
- **WHEN** `i = _SCREENIMAGE(1, 2)` is compiled
- **THEN** it is an error that `_SCREENIMAGE` takes 0 or 4 arguments, as the old compiler rejects it
