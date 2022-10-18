# include "bridge.h"
# include <stdio.h>

void bridge(callback f) {
    printf("C.bridge - callback: %p\n", f);
    (*f)();
}