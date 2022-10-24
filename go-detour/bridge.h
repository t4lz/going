# include <stdint.h>

typedef void (*callback)();
typedef int64_t (*callback_with_args)(int64_t, int64_t, int64_t, int64_t, int64_t, int64_t, int64_t, int64_t);

void bridge(callback f); 
int64_t bridge_with_args(callback_with_args f, int64_t, int64_t, int64_t, int64_t, int64_t, int64_t, int64_t, int64_t); 