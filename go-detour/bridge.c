# include "bridge.h"
# include <stdio.h>

void bridge(callback f) {
    printf("C.bridge - callback: %p\n", f);
    (*f)();
}


int64_t bridge_with_args(callback_with_args f, int64_t arg1 , int64_t arg2) {
    printf("C.bridge - callback: %p\n", f);
    return (*f)(arg1, arg2);
}