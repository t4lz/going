# include "bridge.h"

void bridge(callback f) {
  (*f)();
}