#include <stdint.h>
#include <stdio.h>

int main(void) {
  if (sizeof(void *) != 8 || sizeof(uintptr_t) != 8 || sizeof(uint64_t) != 8) {
    return 1;
  }
  uint64_t value = UINT64_C(0x1122334455667788);
  printf("rv64-lp64:%zu:%llx\n", sizeof(void *), (unsigned long long)value);
  return 0;
}
