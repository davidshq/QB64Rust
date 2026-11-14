# The auto-included constants

<!-- Written by tools/reference/reference.py render; do not edit. -->

The constants QB64pe's auto-included files define, with the text of each value, from the built-in table
(`tools\builtins\builtins.json`, `auto_include`).

## Always defined

Defined in every program, before its first line. 310 constants.

| Name | Type | Value |
|---|---|---|
| `_TRUE` | by its value | `-1` |
| `_FALSE` | by its value | `0` |
| `_LESS` | by its value | `-1` |
| `_EQUAL` | by its value | `0` |
| `_GREATER` | by its value | `1` |
| `_HOST_FAILED` | by its value | `0` |
| `_CLIENT_FAILED` | by its value | `0` |
| `_CONNECTION_FAILED` | by its value | `0` |
| `_INVALID_IMAGE` | by its value | `-1` |
| `_LOADFONT_FAILED` | by its value | `0` |
| `_SNDOPEN_FAILED` | by its value | `0` |
| `_STR_EMPTY` | by its value | `""` |
| `_STR_CRLF` | by its value | `CHR$(13) + CHR$(10)` |
| `_STR_LF` | by its value | `CHR$(10)` |
| `_STR_NAT_EOL` | by its value | `CHR$(13) + CHR$(10)` |
| `_E` | _FLOAT | `EXP(1.0##)` |
| `_LOG_TRACE` | by its value | `1` |
| `_LOG_INFO` | by its value | `2` |
| `_LOG_WARN` | by its value | `3` |
| `_LOG_ERROR` | by its value | `4` |
| `_LOG_NONE` | by its value | `5` |
| `_SECS_IN_MIN` | by its value | `60` |
| `_MINS_IN_HOUR` | by its value | `60` |
| `_HOURS_IN_DAY` | by its value | `24` |
| `_DAYS_IN_WEEK` | by its value | `7` |
| `_DAYS_IN_YEAR` | by its value | `365` |
| `_SECS_IN_HOUR` | by its value | `_MINS_IN_HOUR * _SECS_IN_MIN` |
| `_SECS_IN_DAY` | by its value | `_HOURS_IN_DAY * _SECS_IN_HOUR` |
| `_SECS_IN_WEEK` | by its value | `_DAYS_IN_WEEK * _SECS_IN_DAY` |
| `_SECS_IN_YEAR` | by its value | `_DAYS_IN_YEAR * _SECS_IN_DAY` |
| `_SECS_IN_LEAPYEAR` | by its value | `_SECS_IN_YEAR + _SECS_IN_DAY` |
| `_MINS_IN_DAY` | by its value | `_HOURS_IN_DAY * _MINS_IN_HOUR` |
| `_MINS_IN_WEEK` | by its value | `_DAYS_IN_WEEK * _MINS_IN_DAY` |
| `_MINS_IN_YEAR` | by its value | `_DAYS_IN_YEAR * _MINS_IN_DAY` |
| `_MINS_IN_LEAPYEAR` | by its value | `_MINS_IN_YEAR + _MINS_IN_DAY` |
| `_HOURS_IN_WEEK` | by its value | `_DAYS_IN_WEEK * _HOURS_IN_DAY` |
| `_HOURS_IN_YEAR` | by its value | `_DAYS_IN_YEAR * _HOURS_IN_DAY` |
| `_HOURS_IN_LEAPYEAR` | by its value | `_HOURS_IN_YEAR + _HOURS_IN_DAY` |
| `_DAYS_IN_LEAPYEAR` | by its value | `_DAYS_IN_YEAR + 1` |
| `_WEEKS_IN_YEAR` | by its value | `_DAYS_IN_YEAR / _DAYS_IN_WEEK` |
| `_WEEKS_IN_LEAPYEAR` | by its value | `_DAYS_IN_LEAPYEAR / _DAYS_IN_WEEK` |
| `_ONE_KB` | by its value | `1024` |
| `_ONE_MB` | by its value | `_ONE_KB * _ONE_KB` |
| `_ONE_GB` | by its value | `_ONE_MB * _ONE_KB` |
| `_ONE_TB` | by its value | `_ONE_GB * _ONE_KB` |
| `_ONE_PB` | by its value | `_ONE_TB * _ONE_KB` |
| `_ONE_EB` | by its value | `_ONE_PB * _ONE_KB` |
| `_SIZE_OF_BYTE` | by its value | `1` |
| `_SIZE_OF_INTEGER` | by its value | `2` |
| `_SIZE_OF_LONG` | by its value | `4` |
| `_SIZE_OF_INTEGER64` | by its value | `8` |
| `_SIZE_OF_SINGLE` | by its value | `4` |
| `_SIZE_OF_DOUBLE` | by its value | `8` |
| `_SIZE_OF_FLOAT` | by its value | `32` |
| `_SIZE_OF_OFFSET` | by its value | `8` |
| `_BIT_MIN` | _BIT | `-1` |
| `_BIT_MAX` | _BIT | `0` |
| `_UBIT_MIN` | _UNSIGNED _BIT | `0` |
| `_UBIT_MAX` | _UNSIGNED _BIT | `1` |
| `_BYTE_MIN` | _BYTE | `-128` |
| `_BYTE_MAX` | _BYTE | `127` |
| `_UBYTE_MIN` | _UNSIGNED _BYTE | `0` |
| `_UBYTE_MAX` | _UNSIGNED _BYTE | `255` |
| `_INTEGER_MIN` | INTEGER | `-32768` |
| `_INTEGER_MAX` | INTEGER | `32767` |
| `_UINTEGER_MIN` | _UNSIGNED INTEGER | `0` |
| `_UINTEGER_MAX` | _UNSIGNED INTEGER | `65535` |
| `_LONG_MIN` | LONG | `-2147483648` |
| `_LONG_MAX` | LONG | `2147483647` |
| `_ULONG_MIN` | _UNSIGNED LONG | `0` |
| `_ULONG_MAX` | _UNSIGNED LONG | `4294967295` |
| `_INTEGER64_MIN` | _INTEGER64 | `-9223372036854775808` |
| `_INTEGER64_MAX` | _INTEGER64 | `9223372036854775807` |
| `_UINTEGER64_MIN` | _UNSIGNED _INTEGER64 | `0` |
| `_UINTEGER64_MAX` | _UNSIGNED _INTEGER64 | `18446744073709551615` |
| `_SINGLE_MIN` | SINGLE | `-3.402823E+38` |
| `_SINGLE_MAX` | SINGLE | `3.402823E+38` |
| `_SINGLE_MIN_FRAC` | SINGLE | `1.175494E-38` |
| `_DOUBLE_MIN` | DOUBLE | `-1.797693134862315D+308` |
| `_DOUBLE_MAX` | DOUBLE | `1.797693134862315D+308` |
| `_DOUBLE_MIN_FRAC` | DOUBLE | `2.225073858507201D-308` |
| `_FLOAT_MIN` | _FLOAT | `-1.189731495357231765F+4932` |
| `_FLOAT_MAX` | _FLOAT | `1.189731495357231765F+4932` |
| `_FLOAT_MIN_FRAC` | _FLOAT | `3.362103143112093506F-4932` |
| `_OFFSET_MIN` | _INTEGER64 | `-9223372036854775808` |
| `_OFFSET_MAX` | _INTEGER64 | `9223372036854775807` |
| `_UOFFSET_MIN` | _UNSIGNED _INTEGER64 | `0` |
| `_UOFFSET_MAX` | _UNSIGNED _INTEGER64 | `18446744073709551615` |
| `_ASC_NUL` | by its value | `0` |
| `_CHR_NUL` | by its value | `CHR$(0)` |
| `_ASC_SOH` | by its value | `1` |
| `_CHR_SOH` | by its value | `CHR$(1)` |
| `_ASC_STX` | by its value | `2` |
| `_CHR_STX` | by its value | `CHR$(2)` |
| `_ASC_ETX` | by its value | `3` |
| `_CHR_ETX` | by its value | `CHR$(3)` |
| `_ASC_EOT` | by its value | `4` |
| `_CHR_EOT` | by its value | `CHR$(4)` |
| `_ASC_ENQ` | by its value | `5` |
| `_CHR_ENQ` | by its value | `CHR$(5)` |
| `_ASC_ACK` | by its value | `6` |
| `_CHR_ACK` | by its value | `CHR$(6)` |
| `_ASC_BEL` | by its value | `7` |
| `_CHR_BEL` | by its value | `CHR$(7)` |
| `_ASC_BS` | by its value | `8` |
| `_CHR_BS` | by its value | `CHR$(8)` |
| `_ASC_HT` | by its value | `9` |
| `_CHR_HT` | by its value | `CHR$(9)` |
| `_ASC_LF` | by its value | `10` |
| `_CHR_LF` | by its value | `CHR$(10)` |
| `_ASC_VT` | by its value | `11` |
| `_CHR_VT` | by its value | `CHR$(11)` |
| `_ASC_FF` | by its value | `12` |
| `_CHR_FF` | by its value | `CHR$(12)` |
| `_ASC_CR` | by its value | `13` |
| `_CHR_CR` | by its value | `CHR$(13)` |
| `_ASC_SO` | by its value | `14` |
| `_CHR_SO` | by its value | `CHR$(14)` |
| `_ASC_SI` | by its value | `15` |
| `_CHR_SI` | by its value | `CHR$(15)` |
| `_ASC_DLE` | by its value | `16` |
| `_CHR_DLE` | by its value | `CHR$(16)` |
| `_ASC_DC1` | by its value | `17` |
| `_CHR_DC1` | by its value | `CHR$(17)` |
| `_ASC_DC2` | by its value | `18` |
| `_CHR_DC2` | by its value | `CHR$(18)` |
| `_ASC_DC3` | by its value | `19` |
| `_CHR_DC3` | by its value | `CHR$(19)` |
| `_ASC_DC4` | by its value | `20` |
| `_CHR_DC4` | by its value | `CHR$(20)` |
| `_ASC_NAK` | by its value | `21` |
| `_CHR_NAK` | by its value | `CHR$(21)` |
| `_ASC_SYN` | by its value | `22` |
| `_CHR_SYN` | by its value | `CHR$(22)` |
| `_ASC_ETB` | by its value | `23` |
| `_CHR_ETB` | by its value | `CHR$(23)` |
| `_ASC_CAN` | by its value | `24` |
| `_CHR_CAN` | by its value | `CHR$(24)` |
| `_ASC_EM` | by its value | `25` |
| `_CHR_EM` | by its value | `CHR$(25)` |
| `_ASC_SUB` | by its value | `26` |
| `_CHR_SUB` | by its value | `CHR$(26)` |
| `_ASC_ESC` | by its value | `27` |
| `_CHR_ESC` | by its value | `CHR$(27)` |
| `_ASC_FS` | by its value | `28` |
| `_CHR_FS` | by its value | `CHR$(28)` |
| `_ASC_GS` | by its value | `29` |
| `_CHR_GS` | by its value | `CHR$(29)` |
| `_ASC_RS` | by its value | `30` |
| `_CHR_RS` | by its value | `CHR$(30)` |
| `_ASC_US` | by its value | `31` |
| `_CHR_US` | by its value | `CHR$(31)` |
| `_ASC_DEL` | by its value | `127` |
| `_CHR_DEL` | by its value | `CHR$(127)` |
| `_ASC_SPACE` | by its value | `32` |
| `_CHR_SPACE` | by its value | `CHR$(32)` |
| `_ASC_EXCLAMATION` | by its value | `33` |
| `_CHR_EXCLAMATION` | by its value | `CHR$(33)` |
| `_ASC_QUOTE` | by its value | `34` |
| `_CHR_QUOTE` | by its value | `CHR$(34)` |
| `_ASC_HASH` | by its value | `35` |
| `_CHR_HASH` | by its value | `CHR$(35)` |
| `_ASC_DOLLAR` | by its value | `36` |
| `_CHR_DOLLAR` | by its value | `CHR$(36)` |
| `_ASC_PERCENT` | by its value | `37` |
| `_CHR_PERCENT` | by its value | `CHR$(37)` |
| `_ASC_AMPERSAND` | by its value | `38` |
| `_CHR_AMPERSAND` | by its value | `CHR$(38)` |
| `_ASC_APOSTROPHE` | by its value | `39` |
| `_CHR_APOSTROPHE` | by its value | `CHR$(39)` |
| `_ASC_LEFTBRACKET` | by its value | `40` |
| `_CHR_LEFTBRACKET` | by its value | `CHR$(40)` |
| `_ASC_RIGHTBRACKET` | by its value | `41` |
| `_CHR_RIGHTBRACKET` | by its value | `CHR$(41)` |
| `_ASC_ASTERISK` | by its value | `42` |
| `_CHR_ASTERISK` | by its value | `CHR$(42)` |
| `_ASC_PLUS` | by its value | `43` |
| `_CHR_PLUS` | by its value | `CHR$(43)` |
| `_ASC_COMMA` | by its value | `44` |
| `_CHR_COMMA` | by its value | `CHR$(44)` |
| `_ASC_MINUS` | by its value | `45` |
| `_CHR_MINUS` | by its value | `CHR$(45)` |
| `_ASC_FULLSTOP` | by its value | `46` |
| `_CHR_FULLSTOP` | by its value | `CHR$(46)` |
| `_ASC_FORWARDSLASH` | by its value | `47` |
| `_CHR_FORWARDSLASH` | by its value | `CHR$(47)` |
| `_ASC_COLON` | by its value | `58` |
| `_CHR_COLON` | by its value | `CHR$(58)` |
| `_ASC_SEMICOLON` | by its value | `59` |
| `_CHR_SEMICOLON` | by its value | `CHR$(59)` |
| `_ASC_LESSTHAN` | by its value | `60` |
| `_CHR_LESSTHAN` | by its value | `CHR$(60)` |
| `_ASC_EQUAL` | by its value | `61` |
| `_CHR_EQUAL` | by its value | `CHR$(61)` |
| `_ASC_GREATERTHAN` | by its value | `62` |
| `_CHR_GREATERTHAN` | by its value | `CHR$(62)` |
| `_ASC_QUESTION` | by its value | `63` |
| `_CHR_QUESTION` | by its value | `CHR$(63)` |
| `_ASC_ATSIGN` | by its value | `64` |
| `_CHR_ATSIGN` | by its value | `CHR$(64)` |
| `_ASC_LEFTSQUAREBRACKET` | by its value | `91` |
| `_CHR_LEFTSQUAREBRACKET` | by its value | `CHR$(91)` |
| `_ASC_BACKSLASH` | by its value | `92` |
| `_CHR_BACKSLASH` | by its value | `CHR$(92)` |
| `_ASC_RIGHTSQUAREBRACKET` | by its value | `93` |
| `_CHR_RIGHTSQUAREBRACKET` | by its value | `CHR$(93)` |
| `_ASC_CARET` | by its value | `94` |
| `_CHR_CARET` | by its value | `CHR$(94)` |
| `_ASC_UNDERSCORE` | by its value | `95` |
| `_CHR_UNDERSCORE` | by its value | `CHR$(95)` |
| `_ASC_GRAVE` | by its value | `96` |
| `_CHR_GRAVE` | by its value | `CHR$(96)` |
| `_ASC_LEFTCURLYBRACKET` | by its value | `123` |
| `_CHR_LEFTCURLYBRACKET` | by its value | `CHR$(123)` |
| `_ASC_VERTICALBAR` | by its value | `124` |
| `_CHR_VERTICALBAR` | by its value | `CHR$(124)` |
| `_ASC_RIGHTCURLYBRACKET` | by its value | `125` |
| `_CHR_RIGHTCURLYBRACKET` | by its value | `CHR$(125)` |
| `_ASC_TILDE` | by its value | `126` |
| `_CHR_TILDE` | by its value | `CHR$(126)` |
| `_KEY_LSHIFT` | by its value | `100304` |
| `_KEY_RSHIFT` | by its value | `100303` |
| `_KEY_LCTRL` | by its value | `100306` |
| `_KEY_RCTRL` | by its value | `100305` |
| `_KEY_LALT` | by its value | `100308` |
| `_KEY_RALT` | by its value | `100307` |
| `_KEY_LAPPLE` | by its value | `100310` |
| `_KEY_RAPPLE` | by its value | `100309` |
| `_KEY_F1` | by its value | `15104` |
| `_KEY_F2` | by its value | `15360` |
| `_KEY_F3` | by its value | `15616` |
| `_KEY_F4` | by its value | `15872` |
| `_KEY_F5` | by its value | `16128` |
| `_KEY_F6` | by its value | `16384` |
| `_KEY_F7` | by its value | `16640` |
| `_KEY_F8` | by its value | `16896` |
| `_KEY_F9` | by its value | `17152` |
| `_KEY_F10` | by its value | `17408` |
| `_KEY_F11` | by its value | `34048` |
| `_KEY_F12` | by its value | `34304` |
| `_KEY_INSERT` | by its value | `20992` |
| `_KEY_DELETE` | by its value | `21248` |
| `_KEY_HOME` | by its value | `18176` |
| `_KEY_END` | by its value | `20224` |
| `_KEY_PAGEUP` | by its value | `18688` |
| `_KEY_PAGEDOWN` | by its value | `20736` |
| `_KEY_LEFT` | by its value | `19200` |
| `_KEY_RIGHT` | by its value | `19712` |
| `_KEY_UP` | by its value | `18432` |
| `_KEY_DOWN` | by its value | `20480` |
| `_KEY_BACKSPACE` | by its value | `8` |
| `_KEY_TAB` | by its value | `9` |
| `_KEY_ENTER` | by its value | `13` |
| `_KEY_ESC` | by its value | `27` |
| `_ERR_NONE` | by its value | `0` |
| `_ERR_NEXT_WITHOUT_FOR` | by its value | `1` |
| `_ERR_SYNTAX_ERROR` | by its value | `2` |
| `_ERR_RETURN_WITHOUT_GOSUB` | by its value | `3` |
| `_ERR_OUT_OF_DATA` | by its value | `4` |
| `_ERR_ILLEGAL_FUNCTION_CALL` | by its value | `5` |
| `_ERR_OVERFLOW` | by its value | `6` |
| `_ERR_OUT_OF_MEMORY` | by its value | `7` |
| `_ERR_LABEL_NOT_DEFINED` | by its value | `8` |
| `_ERR_SUBSCRIPT_OUT_OF_RANGE` | by its value | `9` |
| `_ERR_DUPLICATE_DEFINITION` | by its value | `10` |
| `_ERR_DIVISION_BY_ZERO` | by its value | `11` |
| `_ERR_ILLEGAL_IN_DIRECT_MODE` | by its value | `12` |
| `_ERR_TYPE_MISMATCH` | by its value | `13` |
| `_ERR_OUT_OF_STRING_SPACE` | by its value | `14` |
| `_ERR_STRING_FORMULA_TOO_COMPLEX` | by its value | `16` |
| `_ERR_CANNOT_CONTINUE` | by its value | `17` |
| `_ERR_FUNCTION_NOT_DEFINED` | by its value | `18` |
| `_ERR_NO_RESUME` | by its value | `19` |
| `_ERR_RESUME_WITHOUT_ERROR` | by its value | `20` |
| `_ERR_DEVICE_TIMEOUT` | by its value | `24` |
| `_ERR_DEVICE_FAULT` | by its value | `25` |
| `_ERR_FOR_WITHOUT_NEXT` | by its value | `26` |
| `_ERR_OUT_OF_PAPER` | by its value | `27` |
| `_ERR_WHILE_WITHOUT_WEND` | by its value | `29` |
| `_ERR_WEND_WITHOUT_WHILE` | by its value | `30` |
| `_ERR_DUPLICATE_LABEL` | by its value | `33` |
| `_ERR_SUBPROGRAM_NOT_DEFINED` | by its value | `35` |
| `_ERR_ARGUMENT_COUNT_MISMATCH` | by its value | `37` |
| `_ERR_ARRAY_NOT_DEFINED` | by its value | `38` |
| `_ERR_VARIABLE_REQUIRED` | by its value | `40` |
| `_ERR_FIELD_OVERFLOW` | by its value | `50` |
| `_ERR_INTERNAL_ERROR` | by its value | `51` |
| `_ERR_BAD_FILE_NAME_OR_NUMBER` | by its value | `52` |
| `_ERR_FILE_NOT_FOUND` | by its value | `53` |
| `_ERR_BAD_FILE_MODE` | by its value | `54` |
| `_ERR_FILE_ALREADY_OPEN` | by its value | `55` |
| `_ERR_FIELD_STATEMENT_ACTIVE` | by its value | `56` |
| `_ERR_DEVICE_IO_ERROR` | by its value | `57` |
| `_ERR_FILE_ALREADY_EXISTS` | by its value | `58` |
| `_ERR_BAD_RECORD_LENGTH` | by its value | `59` |
| `_ERR_DISK_FULL` | by its value | `61` |
| `_ERR_INPUT_PAST_END_OF_FILE` | by its value | `62` |
| `_ERR_BAD_RECORD_NUMBER` | by its value | `63` |
| `_ERR_BAD_FILE_NAME` | by its value | `64` |
| `_ERR_TOO_MANY_FILES` | by its value | `67` |
| `_ERR_DEVICE_UNAVAILABLE` | by its value | `68` |
| `_ERR_COMMUNICATION_BUFFER_OVERFLOW` | by its value | `69` |
| `_ERR_PERMISSION_DENIED` | by its value | `70` |
| `_ERR_DISK_NOT_READY` | by its value | `71` |
| `_ERR_DISK_MEDIA_ERROR` | by its value | `72` |
| `_ERR_FEATURE_UNAVAILABLE` | by its value | `73` |
| `_ERR_RENAME_ACROSS_DISKS` | by its value | `74` |
| `_ERR_PATH_FILE_ACCESS_ERROR` | by its value | `75` |
| `_ERR_PATH_NOT_FOUND` | by its value | `76` |
| `_ERR_INVALID_HANDLE` | by its value | `258` |

## `$COLOR:0`

Defined after `$COLOR:0`: the 16 colors of the 16-color palette, and `BLINK`. 17 constants.

| Name | Type | Value |
|---|---|---|
| `BLACK` | _UNSIGNED _BYTE | `0` |
| `BLUE` | _UNSIGNED _BYTE | `1` |
| `GREEN` | _UNSIGNED _BYTE | `2` |
| `CYAN` | _UNSIGNED _BYTE | `3` |
| `RED` | _UNSIGNED _BYTE | `4` |
| `MAGENTA` | _UNSIGNED _BYTE | `5` |
| `BROWN` | _UNSIGNED _BYTE | `6` |
| `WHITE` | _UNSIGNED _BYTE | `7` |
| `GRAY` | _UNSIGNED _BYTE | `8` |
| `LIGHTBLUE` | _UNSIGNED _BYTE | `9` |
| `LIGHTGREEN` | _UNSIGNED _BYTE | `10` |
| `LIGHTCYAN` | _UNSIGNED _BYTE | `11` |
| `LIGHTRED` | _UNSIGNED _BYTE | `12` |
| `LIGHTMAGENTA` | _UNSIGNED _BYTE | `13` |
| `YELLOW` | _UNSIGNED _BYTE | `14` |
| `BRIGHTWHITE` | _UNSIGNED _BYTE | `15` |
| `BLINK` | _UNSIGNED _BYTE | `16` |

## `$COLOR:32`

Defined after `$COLOR:32`: 32-bit colors, `_RGB32` values. 270 constants.

| Name | Type | Value |
|---|---|---|
| `ALICEBLUE` | _UNSIGNED LONG | `4293982463` |
| `ALMOND` | _UNSIGNED LONG | `4293910221` |
| `ANTIQUEBRASS` | _UNSIGNED LONG | `4291663221` |
| `ANTIQUEWHITE` | _UNSIGNED LONG | `4294634455` |
| `APRICOT` | _UNSIGNED LONG | `4294826421` |
| `AQUA` | _UNSIGNED LONG | `4278255615` |
| `AQUAMARINE` | _UNSIGNED LONG | `4286578644` |
| `ASPARAGUS` | _UNSIGNED LONG | `4287080811` |
| `ATOMICTANGERINE` | _UNSIGNED LONG | `4294943860` |
| `AZURE` | _UNSIGNED LONG | `4293984255` |
| `BANANAMANIA` | _UNSIGNED LONG | `4294633397` |
| `BEAVER` | _UNSIGNED LONG | `4288643440` |
| `BEIGE` | _UNSIGNED LONG | `4294309340` |
| `BISQUE` | _UNSIGNED LONG | `4294960324` |
| `BITTERSWEET` | _UNSIGNED LONG | `4294802542` |
| `BLACK` | _UNSIGNED LONG | `4278190080` |
| `BLANCHEDALMOND` | _UNSIGNED LONG | `4294962125` |
| `BLIZZARDBLUE` | _UNSIGNED LONG | `4289521134` |
| `BLUE` | _UNSIGNED LONG | `4278190335` |
| `BLUEBELL` | _UNSIGNED LONG | `4288848592` |
| `BLUEGRAY` | _UNSIGNED LONG | `4284914124` |
| `BLUEGREEN` | _UNSIGNED LONG | `4279081146` |
| `BLUEVIOLET` | _UNSIGNED LONG | `4287245282` |
| `BLUSH` | _UNSIGNED LONG | `4292763011` |
| `BRICKRED` | _UNSIGNED LONG | `4291510612` |
| `BROWN` | _UNSIGNED LONG | `4289014314` |
| `BURLYWOOD` | _UNSIGNED LONG | `4292786311` |
| `BURNTORANGE` | _UNSIGNED LONG | `4294934345` |
| `BURNTSIENNA` | _UNSIGNED LONG | `4293557853` |
| `CADETBLUE` | _UNSIGNED LONG | `4284456608` |
| `CANARY` | _UNSIGNED LONG | `4294967193` |
| `CARIBBEANGREEN` | _UNSIGNED LONG | `4280079266` |
| `CARNATIONPINK` | _UNSIGNED LONG | `4294945484` |
| `CERISE` | _UNSIGNED LONG | `4292691090` |
| `CERULEAN` | _UNSIGNED LONG | `4280134870` |
| `CHARTREUSE` | _UNSIGNED LONG | `4286578432` |
| `CHESTNUT` | _UNSIGNED LONG | `4290534744` |
| `CHOCOLATE` | _UNSIGNED LONG | `4291979550` |
| `COPPER` | _UNSIGNED LONG | `4292711541` |
| `CORAL` | _UNSIGNED LONG | `4294934352` |
| `CORNFLOWER` | _UNSIGNED LONG | `4288335595` |
| `CORNFLOWERBLUE` | _UNSIGNED LONG | `4284782061` |
| `CORNSILK` | _UNSIGNED LONG | `4294965468` |
| `COTTONCANDY` | _UNSIGNED LONG | `4294950105` |
| `CRAYOLAAQUAMARINE` | _UNSIGNED LONG | `4286110690` |
| `CRAYOLABLUE` | _UNSIGNED LONG | `4280251902` |
| `CRAYOLABLUEVIOLET` | _UNSIGNED LONG | `4285753021` |
| `CRAYOLABROWN` | _UNSIGNED LONG | `4290013005` |
| `CRAYOLACADETBLUE` | _UNSIGNED LONG | `4289771462` |
| `CRAYOLAFORESTGREEN` | _UNSIGNED LONG | `4285378177` |
| `CRAYOLAGOLD` | _UNSIGNED LONG | `4293379735` |
| `CRAYOLAGOLDENROD` | _UNSIGNED LONG | `4294760821` |
| `CRAYOLAGRAY` | _UNSIGNED LONG | `4287992204` |
| `CRAYOLAGREEN` | _UNSIGNED LONG | `4280069240` |
| `CRAYOLAGREENYELLOW` | _UNSIGNED LONG | `4293978257` |
| `CRAYOLAINDIGO` | _UNSIGNED LONG | `4284315339` |
| `CRAYOLALAVENDER` | _UNSIGNED LONG | `4294751445` |
| `CRAYOLAMAGENTA` | _UNSIGNED LONG | `4294337711` |
| `CRAYOLAMAROON` | _UNSIGNED LONG | `4291311706` |
| `CRAYOLAMIDNIGHTBLUE` | _UNSIGNED LONG | `4279912566` |
| `CRAYOLAORANGE` | _UNSIGNED LONG | `4294931768` |
| `CRAYOLAORANGERED` | _UNSIGNED LONG | `4294912811` |
| `CRAYOLAORCHID` | _UNSIGNED LONG | `4293306583` |
| `CRAYOLAPLUM` | _UNSIGNED LONG | `4287513989` |
| `CRAYOLARED` | _UNSIGNED LONG | `4293795917` |
| `CRAYOLASALMON` | _UNSIGNED LONG | `4294941610` |
| `CRAYOLASEAGREEN` | _UNSIGNED LONG | `4288668351` |
| `CRAYOLASILVER` | _UNSIGNED LONG | `4291675586` |
| `CRAYOLASKYBLUE` | _UNSIGNED LONG | `4286634731` |
| `CRAYOLASPRINGGREEN` | _UNSIGNED LONG | `4293716670` |
| `CRAYOLATANN` | _UNSIGNED LONG | `4294616940` |
| `CRAYOLATHISTLE` | _UNSIGNED LONG | `4293642207` |
| `CRAYOLAVIOLET` | _UNSIGNED LONG | `4287786670` |
| `CRAYOLAYELLOW` | _UNSIGNED LONG | `4294764675` |
| `CRAYOLAYELLOWGREEN` | _UNSIGNED LONG | `4291158916` |
| `CRIMSON` | _UNSIGNED LONG | `4292613180` |
| `CYAN` | _UNSIGNED LONG | `4278255615` |
| `DANDELION` | _UNSIGNED LONG | `4294826861` |
| `DARKBLUE` | _UNSIGNED LONG | `4278190219` |
| `DARKCYAN` | _UNSIGNED LONG | `4278225803` |
| `DARKGOLDENROD` | _UNSIGNED LONG | `4290283019` |
| `DARKGRAY` | _UNSIGNED LONG | `4289309097` |
| `DARKGREEN` | _UNSIGNED LONG | `4278215680` |
| `DARKKHAKI` | _UNSIGNED LONG | `4290623339` |
| `DARKMAGENTA` | _UNSIGNED LONG | `4287299723` |
| `DARKOLIVEGREEN` | _UNSIGNED LONG | `4283788079` |
| `DARKORANGE` | _UNSIGNED LONG | `4294937600` |
| `DARKORCHID` | _UNSIGNED LONG | `4288230092` |
| `DARKRED` | _UNSIGNED LONG | `4287299584` |
| `DARKSALMON` | _UNSIGNED LONG | `4293498490` |
| `DARKSEAGREEN` | _UNSIGNED LONG | `4287609999` |
| `DARKSLATEBLUE` | _UNSIGNED LONG | `4282924427` |
| `DARKSLATEGRAY` | _UNSIGNED LONG | `4281290575` |
| `DARKTURQUOISE` | _UNSIGNED LONG | `4278243025` |
| `DARKVIOLET` | _UNSIGNED LONG | `4287889619` |
| `DEEPPINK` | _UNSIGNED LONG | `4294907027` |
| `DEEPSKYBLUE` | _UNSIGNED LONG | `4278239231` |
| `DENIM` | _UNSIGNED LONG | `4281035972` |
| `DESERTSAND` | _UNSIGNED LONG | `4293905848` |
| `DIMGRAY` | _UNSIGNED LONG | `4285098345` |
| `DODGERBLUE` | _UNSIGNED LONG | `4280193279` |
| `EGGPLANT` | _UNSIGNED LONG | `4285419872` |
| `ELECTRICLIME` | _UNSIGNED LONG | `4291755805` |
| `FERN` | _UNSIGNED LONG | `4285643896` |
| `FIREBRICK` | _UNSIGNED LONG | `4289864226` |
| `FLORALWHITE` | _UNSIGNED LONG | `4294966000` |
| `FORESTGREEN` | _UNSIGNED LONG | `4280453922` |
| `FUCHSIA` | _UNSIGNED LONG | `4290995397` |
| `FUZZYWUZZY` | _UNSIGNED LONG | `4291585638` |
| `GAINSBORO` | _UNSIGNED LONG | `4292664540` |
| `GHOSTWHITE` | _UNSIGNED LONG | `4294506751` |
| `GOLD` | _UNSIGNED LONG | `4294956800` |
| `GOLDENROD` | _UNSIGNED LONG | `4292519200` |
| `GRANNYSMITHAPPLE` | _UNSIGNED LONG | `4289258656` |
| `GRAY` | _UNSIGNED LONG | `4286611584` |
| `GREEN` | _UNSIGNED LONG | `4278222848` |
| `GREENBLUE` | _UNSIGNED LONG | `4279329972` |
| `GREENYELLOW` | _UNSIGNED LONG | `4289593135` |
| `HONEYDEW` | _UNSIGNED LONG | `4293984240` |
| `HOTMAGENTA` | _UNSIGNED LONG | `4294909390` |
| `HOTPINK` | _UNSIGNED LONG | `4294928820` |
| `INCHWORM` | _UNSIGNED LONG | `4289915997` |
| `INDIANRED` | _UNSIGNED LONG | `4291648604` |
| `INDIGO` | _UNSIGNED LONG | `4283105410` |
| `IVORY` | _UNSIGNED LONG | `4294967280` |
| `JAZZBERRYJAM` | _UNSIGNED LONG | `4291442535` |
| `JUNGLEGREEN` | _UNSIGNED LONG | `4282101903` |
| `KHAKI` | _UNSIGNED LONG | `4293977740` |
| `LASERLEMON` | _UNSIGNED LONG | `4294901282` |
| `LAVENDER` | _UNSIGNED LONG | `4293322490` |
| `LAVENDERBLUSH` | _UNSIGNED LONG | `4294963445` |
| `LAWNGREEN` | _UNSIGNED LONG | `4286381056` |
| `LEMONCHIFFON` | _UNSIGNED LONG | `4294965965` |
| `LEMONYELLOW` | _UNSIGNED LONG | `4294964303` |
| `LIGHTBLUE` | _UNSIGNED LONG | `4289583334` |
| `LIGHTCORAL` | _UNSIGNED LONG | `4293951616` |
| `LIGHTCYAN` | _UNSIGNED LONG | `4292935679` |
| `LIGHTGOLDENRODYELLOW` | _UNSIGNED LONG | `4294638290` |
| `LIGHTGRAY` | _UNSIGNED LONG | `4292072403` |
| `LIGHTGREEN` | _UNSIGNED LONG | `4287688336` |
| `LIGHTPINK` | _UNSIGNED LONG | `4294948545` |
| `LIGHTSALMON` | _UNSIGNED LONG | `4294942842` |
| `LIGHTSEAGREEN` | _UNSIGNED LONG | `4280332970` |
| `LIGHTSKYBLUE` | _UNSIGNED LONG | `4287090426` |
| `LIGHTSLATEGRAY` | _UNSIGNED LONG | `4286023833` |
| `LIGHTSTEELBLUE` | _UNSIGNED LONG | `4289774814` |
| `LIGHTYELLOW` | _UNSIGNED LONG | `4294967264` |
| `LIME` | _UNSIGNED LONG | `4278255360` |
| `LIMEGREEN` | _UNSIGNED LONG | `4281519410` |
| `LINEN` | _UNSIGNED LONG | `4294635750` |
| `MACARONIANDCHEESE` | _UNSIGNED LONG | `4294950280` |
| `MAGENTA` | _UNSIGNED LONG | `4294902015` |
| `MAGICMINT` | _UNSIGNED LONG | `4289392849` |
| `MAHOGANY` | _UNSIGNED LONG | `4291643980` |
| `MAIZE` | _UNSIGNED LONG | `4293775772` |
| `MANATEE` | _UNSIGNED LONG | `4288125610` |
| `MANGOTANGO` | _UNSIGNED LONG | `4294935107` |
| `MAROON` | _UNSIGNED LONG | `4286578688` |
| `MAUVELOUS` | _UNSIGNED LONG | `4293892266` |
| `MEDIUMAQUAMARINE` | _UNSIGNED LONG | `4284927402` |
| `MEDIUMBLUE` | _UNSIGNED LONG | `4278190285` |
| `MEDIUMORCHID` | _UNSIGNED LONG | `4290401747` |
| `MEDIUMPURPLE` | _UNSIGNED LONG | `4287852763` |
| `MEDIUMSEAGREEN` | _UNSIGNED LONG | `4282168177` |
| `MEDIUMSLATEBLUE` | _UNSIGNED LONG | `4286277870` |
| `MEDIUMSPRINGGREEN` | _UNSIGNED LONG | `4278254234` |
| `MEDIUMTURQUOISE` | _UNSIGNED LONG | `4282962380` |
| `MEDIUMVIOLETRED` | _UNSIGNED LONG | `4291237253` |
| `MELON` | _UNSIGNED LONG | `4294818996` |
| `MIDNIGHTBLUE` | _UNSIGNED LONG | `4279834992` |
| `MINTCREAM` | _UNSIGNED LONG | `4294311930` |
| `MISTYROSE` | _UNSIGNED LONG | `4294960353` |
| `MOCCASIN` | _UNSIGNED LONG | `4294960309` |
| `MOUNTAINMEADOW` | _UNSIGNED LONG | `4281383567` |
| `MULBERRY` | _UNSIGNED LONG | `4291120012` |
| `NAVAJOWHITE` | _UNSIGNED LONG | `4294958765` |
| `NAVY` | _UNSIGNED LONG | `4278190208` |
| `NAVYBLUE` | _UNSIGNED LONG | `4279858386` |
| `NEONCARROT` | _UNSIGNED LONG | `4294943555` |
| `OLDLACE` | _UNSIGNED LONG | `4294833638` |
| `OLIVE` | _UNSIGNED LONG | `4286611456` |
| `OLIVEDRAB` | _UNSIGNED LONG | `4285238819` |
| `OLIVEGREEN` | _UNSIGNED LONG | `4290426988` |
| `ORANGE` | _UNSIGNED LONG | `4294944000` |
| `ORANGERED` | _UNSIGNED LONG | `4294919424` |
| `ORANGEYELLOW` | _UNSIGNED LONG | `4294497640` |
| `ORCHID` | _UNSIGNED LONG | `4292505814` |
| `OUTERSPACE` | _UNSIGNED LONG | `4282468940` |
| `OUTRAGEOUSORANGE` | _UNSIGNED LONG | `4294929994` |
| `PACIFICBLUE` | _UNSIGNED LONG | `4280068553` |
| `PALEGOLDENROD` | _UNSIGNED LONG | `4293847210` |
| `PALEGREEN` | _UNSIGNED LONG | `4288215960` |
| `PALETURQUOISE` | _UNSIGNED LONG | `4289720046` |
| `PALEVIOLETRED` | _UNSIGNED LONG | `4292571283` |
| `PAPAYAWHIP` | _UNSIGNED LONG | `4294963157` |
| `PEACH` | _UNSIGNED LONG | `4294954923` |
| `PEACHPUFF` | _UNSIGNED LONG | `4294957753` |
| `PERIWINKLE` | _UNSIGNED LONG | `4291154150` |
| `PERU` | _UNSIGNED LONG | `4291659071` |
| `PIGGYPINK` | _UNSIGNED LONG | `4294827494` |
| `PINEGREEN` | _UNSIGNED LONG | `4279599224` |
| `PINK` | _UNSIGNED LONG | `4294951115` |
| `PINKFLAMINGO` | _UNSIGNED LONG | `4294735101` |
| `PINKSHERBET` | _UNSIGNED LONG | `4294414247` |
| `PLUM` | _UNSIGNED LONG | `4292714717` |
| `POWDERBLUE` | _UNSIGNED LONG | `4289781990` |
| `PURPLE` | _UNSIGNED LONG | `4286578816` |
| `PURPLEHEART` | _UNSIGNED LONG | `4285809352` |
| `PURPLEMOUNTAINSMAJESTY` | _UNSIGNED LONG | `4288512442` |
| `PURPLEPIZZAZZ` | _UNSIGNED LONG | `4294856410` |
| `RADICALRED` | _UNSIGNED LONG | `4294920556` |
| `RAWSIENNA` | _UNSIGNED LONG | `4292250201` |
| `RAWUMBER` | _UNSIGNED LONG | `4285614883` |
| `RAZZLEDAZZLEROSE` | _UNSIGNED LONG | `4294920400` |
| `RAZZMATAZZ` | _UNSIGNED LONG | `4293076331` |
| `RED` | _UNSIGNED LONG | `4294901760` |
| `REDORANGE` | _UNSIGNED LONG | `4294923081` |
| `REDVIOLET` | _UNSIGNED LONG | `4290790543` |
| `ROBINSEGGBLUE` | _UNSIGNED LONG | `4280274635` |
| `ROSYBROWN` | _UNSIGNED LONG | `4290547599` |
| `ROYALBLUE` | _UNSIGNED LONG | `4282477025` |
| `ROYALPURPLE` | _UNSIGNED LONG | `4286075305` |
| `SADDLEBROWN` | _UNSIGNED LONG | `4287317267` |
| `SALMON` | _UNSIGNED LONG | `4294606962` |
| `SANDYBROWN` | _UNSIGNED LONG | `4294222944` |
| `SCARLET` | _UNSIGNED LONG | `4294715463` |
| `SCREAMINGREEN` | _UNSIGNED LONG | `4285988730` |
| `SEAGREEN` | _UNSIGNED LONG | `4281240407` |
| `SEASHELL` | _UNSIGNED LONG | `4294964718` |
| `SEPIA` | _UNSIGNED LONG | `4289030479` |
| `SHADOW` | _UNSIGNED LONG | `4287265117` |
| `SHAMROCK` | _UNSIGNED LONG | `4282764962` |
| `SHOCKINGPINK` | _UNSIGNED LONG | `4294672125` |
| `SIENNA` | _UNSIGNED LONG | `4288696877` |
| `SILVER` | _UNSIGNED LONG | `4290822336` |
| `SKYBLUE` | _UNSIGNED LONG | `4287090411` |
| `SLATEBLUE` | _UNSIGNED LONG | `4285160141` |
| `SLATEGRAY` | _UNSIGNED LONG | `4285563024` |
| `SNOW` | _UNSIGNED LONG | `4294966010` |
| `SPRINGGREEN` | _UNSIGNED LONG | `4278255487` |
| `STEELBLUE` | _UNSIGNED LONG | `4282811060` |
| `SUNGLOW` | _UNSIGNED LONG | `4294954824` |
| `SUNSETORANGE` | _UNSIGNED LONG | `4294794835` |
| `TANN` | _UNSIGNED LONG | `4291998860` |
| `TEAL` | _UNSIGNED LONG | `4278222976` |
| `TEALBLUE` | _UNSIGNED LONG | `4279805877` |
| `THISTLE` | _UNSIGNED LONG | `4292394968` |
| `TICKLEMEPINK` | _UNSIGNED LONG | `4294740396` |
| `TIMBERWOLF` | _UNSIGNED LONG | `4292597714` |
| `TOMATO` | _UNSIGNED LONG | `4294927175` |
| `TROPICALRAINFOREST` | _UNSIGNED LONG | `4279730285` |
| `TUMBLEWEED` | _UNSIGNED LONG | `4292782728` |
| `TURQUOISE` | _UNSIGNED LONG | `4282441936` |
| `TURQUOISEBLUE` | _UNSIGNED LONG | `4286045671` |
| `UNMELLOWYELLOW` | _UNSIGNED LONG | `4294967142` |
| `VIOLET` | _UNSIGNED LONG | `4293821166` |
| `VIOLETBLUE` | _UNSIGNED LONG | `4281486002` |
| `VIOLETRED` | _UNSIGNED LONG | `4294398868` |
| `VIVIDTANGERINE` | _UNSIGNED LONG | `4294942857` |
| `VIVIDVIOLET` | _UNSIGNED LONG | `4287582365` |
| `WHEAT` | _UNSIGNED LONG | `4294303411` |
| `WHITE` | _UNSIGNED LONG | `4294967295` |
| `WHITESMOKE` | _UNSIGNED LONG | `4294309365` |
| `WILDBLUEYONDER` | _UNSIGNED LONG | `4288851408` |
| `WILDSTRAWBERRY` | _UNSIGNED LONG | `4294919076` |
| `WILDWATERMELON` | _UNSIGNED LONG | `4294732933` |
| `WISTERIA` | _UNSIGNED LONG | `4291667166` |
| `YELLOW` | _UNSIGNED LONG | `4294967040` |
| `YELLOWGREEN` | _UNSIGNED LONG | `4288335154` |
| `YELLOWORANGE` | _UNSIGNED LONG | `4294946370` |
