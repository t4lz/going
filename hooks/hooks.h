#include <stdint.h>

// This is necessary for the Go detours to be able to call the rust detours.

void rust_detour();

int64_t rust_detour_with_args(int64_t arg1,
                              int64_t arg2,
                              int64_t arg3,
                              int64_t arg4,
                              int64_t arg5,
                              int64_t arg6,
                              int64_t arg7,
                              int64_t arg8);
