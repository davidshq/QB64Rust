// Header library of the verification programs v37_* (DECLARE LIBRARY "v37_lib/v37h").
// Included into the program's C++ by the old compiler, so C++ is allowed here. The old compiler includes
// it once for each block that names it, so it needs the guard (v37_x16 measures a header without one).
#pragma once
#include <stdint.h>
#include <string.h>

// One function per numeric type: by value it returns the value plus one, by reference it adds one.
int8_t h_b(int8_t v) { return v + 1; }
void h_pb(int8_t *p) { *p += 1; }
uint8_t h_ub(uint8_t v) { return v + 1; }
void h_pub(uint8_t *p) { *p += 1; }
int16_t h_i(int16_t v) { return v + 1; }
void h_pi(int16_t *p) { *p += 1; }
uint16_t h_ui(uint16_t v) { return v + 1; }
void h_pui(uint16_t *p) { *p += 1; }
int32_t h_l(int32_t v) { return v + 1; }
void h_pl(int32_t *p) { *p += 1; }
uint32_t h_ul(uint32_t v) { return v + 1; }
void h_pul(uint32_t *p) { *p += 1; }
int64_t h_ll(int64_t v) { return v + 1; }
void h_pll(int64_t *p) { *p += 1; }
uint64_t h_ull(uint64_t v) { return v + 1; }
void h_pull(uint64_t *p) { *p += 1; }
float h_s(float v) { return v + 1; }
void h_ps(float *p) { *p += 1; }
double h_d(double v) { return v + 1; }
void h_pd(double *p) { *p += 1; }
long double h_f(long double v) { return v + 1; }
void h_pf(long double *p) { *p += 1; }
intptr_t h_o(intptr_t v) { return v + 1; }
void h_po(intptr_t *p) { *p += 1; }
uintptr_t h_uo(uintptr_t v) { return v + 1; }
void h_puo(uintptr_t *p) { *p += 1; }

// Two arguments, and a SUB with none.
int32_t h_add(int32_t a, int32_t b) { return a + b; }
static int32_t h_count = 0;
void h_tick(void) { h_count++; }
int32_t h_ticks(void) { return h_count; }

// Pointers: whether an argument arrived as NULL, the value behind an address.
int32_t h_isnull(const void *p) { return p == NULL; }
int32_t h_peekl(intptr_t p) { return *(int32_t *)p; }

// Strings: the length C sees (up to the first NUL), a change made in place, a returned string, NULL.
int32_t h_len(const char *s) { return (int32_t)strlen(s); }
void h_upper(char *s) {
    for (; *s; s++)
        if (*s >= 'a' && *s <= 'z') *s -= 32;
}
const char *h_hello(void) { return "hello"; }
const char *h_empty(void) { return ""; }
const char *h_nullstr(void) { return NULL; }

// A TYPE passed as void*: struct of two LONGs.
struct h_pt { int32_t x, y; };
int32_t h_ptsum(void *p) { return ((struct h_pt *)p)->x + ((struct h_pt *)p)->y; }
void h_ptset(void *p) { ((struct h_pt *)p)->x = 7; }

// C++ names reached through ALIAS: a template, a namespace, an overload.
template <typename T> T h_max(T a, T b) { return a > b ? a : b; }
namespace hns {
int32_t twice(int32_t v) { return v * 2; }
}
int32_t h_over(int32_t v) { return 1; }
int32_t h_over(double v) { return 2; }
