#include <stdio.h>
int loop_c() {
    unsigned int x;
    for (x = 0; x < 100000; x++) {
      printf("%i", x);
    }
    return 0;
}
