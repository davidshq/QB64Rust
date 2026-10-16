// C object file for the verification programs v37_*: built into v37o.o by build_libs.py, linked by
// DECLARE LIBRARY "./v37o" (an object file is linked whole, so its names differ from v37c.c's).
#include <stdint.h>

static int32_t o_count = 0;
void o_tick(void) { o_count++; }
int32_t o_ticks(void) { return o_count; }
