# Checksums, compression and base64

Seven functions work on the bytes of a string: two checksums, a digest, compression and its reverse, and base64 in
both directions.

The pages of this area: [_CRC32](../entries/_crc32.md), [_ADLER32](../entries/_adler32.md),
[_MD5$](../entries/_md5-str.md), [_DEFLATE$](../entries/_deflate-str.md), [_INFLATE$](../entries/_inflate-str.md),
[_BASE64ENCODE$](../entries/_base64encode-str.md), [_BASE64DECODE$](../entries/_base64decode-str.md).

## The functions

`_CRC32` and `_ADLER32` give an `_UNSIGNED LONG`; the others give a string. Each takes any string, with any bytes in
it.
<!-- src: language/data-processing "Checksums, compression and base64"; tests\corpus\slice\s109_data_processing -->

`_MD5$` gives 32 upper-case hexadecimal digits.
<!-- src: language/data-processing "_MD5$"; tests\reference\examples\_md5-str_1 -->

`_DEFLATE$` is zlib's deflate, at a level from 0 to 9; `_INFLATE$` undoes it, and with a size gives that many bytes.
<!-- src: language/data-processing "Checksums, compression and base64"; tests\corpus\slice\s109_data_processing -->

`_BASE64DECODE$` of a string that is not base64, or whose padding is missing, gives the empty string: a program cannot
tell it from the code of an empty string.
<!-- src: tests\corpus\slice\s109_data_processing; tests\reference\examples\_base64decode-str_1 -->

## The runtime part

A program that names any of these functions is linked with libqb's data-processing part, as QB64pe's build is; one
that names none is not.
<!-- src: language/data-processing "The runtime part of the data-processing functions" -->
