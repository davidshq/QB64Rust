// Verification v66_d09, v66_d10: SUBs of a DECLARE LIBRARY block as event handlers.
static int32_t v66_count = 0;
void v66_tick() { v66_count++; }
void v66_add(int32_t n) { v66_count += n; }
int32_t v66_get() { return v66_count; }
