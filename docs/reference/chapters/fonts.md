# Fonts

Text is drawn in a font: one of the built-in fonts, or one loaded from a file. Unicode text is drawn with
`_UPRINTSTRING`.

The pages of this area: [_LOADFONT](../entries/_loadfont.md), [_FONT](../entries/_font.md),
[_FONTWIDTH](../entries/_fontwidth.md), [_PRINTSTRING](../entries/_printstring.md),
[_PRINTMODE](../entries/_printmode.md), [_UPRINTSTRING](../entries/_uprintstring.md),
[_UFONTHEIGHT](../entries/_ufontheight.md), [_UCHARPOS](../entries/_ucharpos.md),
[_MAPUNICODE](../entries/_mapunicode.md).

## Built-in and loaded fonts

`_FONT 8`, `_FONT 14` and `_FONT 16` choose libqb's built-in fonts for the destination, and `_FONTWIDTH` and
`_FONTHEIGHT` measure the font in use.
<!-- src: language/fonts "Built-in fonts"; tests\reference\examples\_font_1 -->

`_LOADFONT` loads a font file from the machine and gives its handle, and `_FREEFONT` frees it.
<!-- src: language/fonts "Fonts loaded from Windows" -->

## Unicode

`_UPRINTSTRING` draws Unicode text, `_UPRINTWIDTH` measures it and `_UCHARPOS` gives each character's place.
`_MAPUNICODE` sets which code point an ASCII character shows.
<!-- src: language/fonts "Unicode text and the print mode"; tests\reference\examples\_uprintstring_1 -->
