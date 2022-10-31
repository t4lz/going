# include <stdint.h>

struct raw_syscall_result {
    uintptr_t res1;
    uintptr_t res2;
    uintptr_t errno;
};

typedef struct raw_syscall_result (*syscall_callback)(uintptr_t, uintptr_t, uintptr_t, uintptr_t);

struct raw_syscall_result syscall_bridge(syscall_callback, uintptr_t, uintptr_t, uintptr_t, uintptr_t);