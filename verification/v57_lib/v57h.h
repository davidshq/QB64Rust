// Header library of the verification programs v57_l* (DECLARE LIBRARY "v57_lib/v57h").
#pragma once
#include <stdint.h>
#include <stdio.h>

int32_t h_l(int32_t v) { return v + 1; }
void h_v(int32_t v) { printf("library SUB h_v %d\n", (int)v); fflush(stdout); }
