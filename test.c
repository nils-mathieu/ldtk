#include <unistd.h>

int main(void) {
    write(STDERR_FILENO, "err\n", 4);
    write(STDOUT_FILENO, "out\n", 4);
}
