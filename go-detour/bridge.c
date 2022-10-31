# include "bridge.h"
# include <stdio.h>

struct raw_syscall_result syscall_bridge(syscall_callback f, uintptr_t trap, uintptr_t arg1, uintptr_t arg2, uintptr_t arg3) {
    printf("C.bridge - callback: %p\n", f);
    return (*f)(trap, arg1, arg2, arg3);
}