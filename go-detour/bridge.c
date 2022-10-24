# include "bridge.h"
# include <stdio.h>

void bridge(callback f) {
    printf("C.bridge - callback: %p\n", f);
    (*f)();
}


int64_t bridge_with_args(callback_with_args f, int64_t arg1 , int64_t arg2, int64_t arg3, int64_t arg4, int64_t arg5, int64_t arg6, int64_t arg7, int64_t arg8) {
    printf("C.bridge - callback: %p\n", f);
    return (*f)(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
}